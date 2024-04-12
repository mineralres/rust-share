use base::*;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
mod executor;
mod fronts;
use log::info;

#[derive(Parser)]
#[clap()]
struct Opts {
    config: String,
}

#[tokio::main]
async fn main() {
    init_logger();
    let opts: Opts = Opts::parse();
    let conf = GatewayConfig::load(&opts.config).unwrap();
    let mut ve = vec![];
    for e in conf.executors.into_iter().filter(|x| x.enabled) {
        info!("{:?}", e);
        let (tx, rx) = oneshot::channel::<i32>();
        ve.push(rx);
        tokio::spawn(async move {
            let mut ex = executor::Executor::new();
            if e.r#type == "ctp_futures" {
                let cmc = ctp_futures::route::new_md_cache(&e.md_account);
                for ca in e.accounts.iter() {
                    let cmc = Arc::clone(&cmc);
                    let ca = ca.clone();
                    let (tx, rx) = mpsc::channel::<(
                        base::ReqMessage,
                        tokio::sync::oneshot::Sender<RspMessage>,
                    )>(1000);
                    ex.sorted_accounts.push(executor::TraderHandle {
                        account: ca.account.clone(),
                        tx,
                    });
                    tokio::spawn(ctp_futures::route::run_trader(ca, cmc, rx));
                }
            }
            if e.r#type == "tora_stock" {
                let cmc = tora_stock::route::new_md_cache(&e.md_account);
                for ca in e.accounts.iter() {
                    let cmc = Arc::clone(&cmc);
                    let ca = ca.clone();
                    let (tx, rx) = mpsc::channel::<(
                        base::ReqMessage,
                        tokio::sync::oneshot::Sender<RspMessage>,
                    )>(1000);
                    ex.sorted_accounts.push(executor::TraderHandle {
                        account: ca.account.clone(),
                        tx,
                    });
                    tokio::spawn(tora_stock::route::run_trader(ca, cmc, rx));
                }
            }
            fronts::http::serve(e.clone(), Arc::new(Mutex::new(ex))).await;
            let _ = tx.send(1);
        });
    }
    for rx in ve.into_iter() {
        let _ = rx.await;
    }
}
