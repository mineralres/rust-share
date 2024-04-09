use base::*;
use clap::Parser;
use gateway::{executor, fronts, Config};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Parser)]
#[clap()]
struct Opts {
    config: String,
}

#[tokio::main]
async fn main() {
    init_logger();
    let mut ex = executor::Executor::new();
    let opts: Opts = Opts::parse();
    let conf = Config::load(&opts.config).unwrap();
    let cmc = executor::CtpMdCache::new(&conf.ctp_md_account);
    for ca in conf.ctp_accounts.iter() {
        let cmc = Arc::clone(&cmc);
        let ca = ca.clone();
        let (tx, rx) = mpsc::channel::<(
            executor::ReqMessage,
            tokio::sync::oneshot::Sender<executor::RspMessage>,
        )>(1000);
        ex.sorted_accounts.push(executor::TraderHandle {
            account: ca.account.clone(),
            tx,
        });
        tokio::spawn(executor::Executor::run_trader_daemon(ca, cmc, rx));
    }
    fronts::http::serve(conf.clone(), Arc::new(Mutex::new(ex))).await;
}
