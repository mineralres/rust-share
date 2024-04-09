use clap::Parser;
use log::info;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

mod executor;
mod fronts;
mod position;
mod state;
use base::*;
mod error;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub http_port: u16,
    pub https_port: u16,
    pub ctp_accounts: Vec<ctp_futures::CtpAccountConfig>,
    pub ctp_md_account: ctp_futures::CtpAccountConfig,
}

impl Config {
    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let c = serde_json::from_reader(reader)?;
        Ok(c)
    }
}

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
