use base::*;
use clap::Parser;
use log::info;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub mod error;
pub mod executor;
pub mod fronts;
pub mod position;
pub mod state;

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
