use base::state::{ReqMessage, RspMessage};
use base::*;
use clap::Parser;
use log::{error, info};
use rust_share::gateway::executor;
use rust_share::gateway::fronts;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

#[derive(Parser)]
#[clap()]
struct Opts {
    config: String,
}

/// 推送重要的交易通知
pub struct MyHttpLogger {
    pub buffer: Arc<std::sync::Mutex<std::collections::VecDeque<(String, String)>>>,
    pub notify: Arc<tokio::sync::Notify>,
}

impl MyHttpLogger {
    fn new(broker_id: String, account: String, host: String, token: String) -> Self {
        let notify = Arc::new(tokio::sync::Notify::new());
        let notify2 = notify.clone();
        let buffer = Arc::new(std::sync::Mutex::new(VecDeque::new()));
        let buffer2 = Arc::clone(&buffer);
        tokio::spawn(async move {
            let url = format!("{host}/log/info");
            loop {
                notify2.notified().await;
                let l = { buffer2.lock().unwrap().pop_front() };
                if let Some((title, content)) = l {
                    info!("url={url}");
                    #[derive(Debug, serde::Serialize, serde::Deserialize)]
                    struct ReqLogInfo {
                        pub title: String,
                        pub content: String,
                        pub token: String,
                    }
                    let req = ReqLogInfo {
                        title: title,
                        content: format!("[{}:{}] {content}", broker_id, account),
                        token: token.clone(),
                    };

                    let client = reqwest::Client::new();
                    let resp = client.post(&url).json(&req).send().await;
                    if let Err(e) = resp {
                        error!("{e}");
                    }
                }
            }
        });
        Self { buffer, notify }
    }
}

impl base::state::TradingLoger for MyHttpLogger {
    fn info(&self, title: &str, l: &str) {
        self.buffer
            .lock()
            .unwrap()
            .push_back((title.into(), l.into()));
        self.notify.notify_one();
    }
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
        let logger_config = conf.logger.clone();
        tokio::spawn(async move {
            let mut ex = executor::Executor::new();
            if e.r#type == "ctp_futures" {
                let cmc = ctp_futures::route::new_md_cache(&e.md_account);
                for ca in e.accounts.iter() {
                    let cmc = Arc::clone(&cmc);
                    let ca = ca.clone();
                    let (tx, rx) = mpsc::channel::<(
                        ReqMessage,
                        tokio::sync::oneshot::Sender<RspMessage>,
                    )>(1000);
                    ex.sorted_accounts.push(executor::TraderHandle {
                        account: ca.account.clone(),
                        tx,
                    });
                    let logger = MyHttpLogger::new(
                        ca.broker_id.clone(),
                        ca.account.clone(),
                        logger_config.host.clone(),
                        logger_config.token.clone(),
                    );
                    tokio::spawn(async move {
                        if let Err(e) =
                            ctp_futures::route::run_trader(ca, cmc, rx, Box::new(logger)).await
                        {
                            error!("ctp_futures trader exit {e}");
                        }
                    });
                }
            }
            if e.r#type == "tora_stock" {
                let cmc = tora_stock::route::new_md_cache(&e.md_account);
                for ca in e.accounts.iter() {
                    let cmc = Arc::clone(&cmc);
                    let ca = ca.clone();
                    let (tx, rx) = mpsc::channel::<(
                        ReqMessage,
                        tokio::sync::oneshot::Sender<RspMessage>,
                    )>(1000);
                    ex.sorted_accounts.push(executor::TraderHandle {
                        account: ca.account.clone(),
                        tx,
                    });
                    let logger = MyHttpLogger::new(
                        ca.broker_id.clone(),
                        ca.account.clone(),
                        logger_config.host.clone(),
                        logger_config.token.clone(),
                    );
                    tokio::spawn(async move {
                        if let Err(e) =
                            tora_stock::route::run_trader(ca, cmc, rx, Box::new(logger)).await
                        {
                            error!("tora_stock trader exit {e}");
                        }
                    });
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
