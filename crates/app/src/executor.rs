#![allow(non_upper_case_globals)]

use base::error::Error;
use base::*;
use log::error;
use tokio::sync::{mpsc, oneshot};
/// executor for all account
/// 行情存放在hm_md中, 按需取
/// trader运行后,接收set_symbol_target指令
pub struct Executor {
    pub sorted_accounts: Vec<TraderHandle>,
}

pub struct TraderHandle {
    pub account: String,
    pub tx: mpsc::Sender<(ReqMessage, oneshot::Sender<RspMessage>)>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            sorted_accounts: vec![],
        }
    }

    pub async fn query(&self, account: &str, msg: ReqMessage) -> Result<RspMessage, Error> {
        match self.sorted_accounts.iter().find(|t| t.account == account) {
            Some(th) => {
                let (tx, rx) = oneshot::channel::<RspMessage>();
                th.tx
                    .send((msg, tx))
                    .await
                    .map_err(|_e| Error::MpscSendErr)?;
                match tokio::time::timeout(tokio::time::Duration::from_secs(5), rx).await {
                    Ok(res) => match res {
                        Ok(v) => Ok(v),
                        Err(_) => Err(Error::CtpQueryTimeout),
                    },
                    Err(_) => {
                        error!("did not receive value within 5 seconds");
                        Err(Error::CtpQueryTimeout)
                    }
                }
            }
            None => Err(Error::AccountNotFound),
        }
    }
}
