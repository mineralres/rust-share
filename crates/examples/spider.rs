use clap::Parser;
use log::info;
use serde::{Deserialize, Serialize};
use spider::*;
use std::fs::File;
use std::io::Write;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}

// 交易配置
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpiderConfig {
    pub name: String,
}

#[derive(Parser)]
#[clap()]
pub struct Opts {
    #[clap(short, long, default_value = "default.conf")]
    pub config: String,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Clone)]
pub enum SubCommand {
    GetCommonDataList(GetCommonDataListCommand),
}

/// mysteel 常规数据
#[derive(Parser, Clone)]
pub struct GetCommonDataListCommand {
    pub category_code: String,
    pub chart_code: String,
    #[clap(default_value = "")]
    pub output_path: String,
}

#[tokio::main]
async fn main() {
    init_logger();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::GetCommonDataList(cmd) => {
            get_common_data_list(&cmd).await.unwrap();
        }
    }
}

/// category_code 一般是品种 RB, chart_code是  rbSupplyYield
///
async fn get_common_data_list(cmd: &GetCommonDataListCommand) -> std::io::Result<()> {
    rust_share_util::check_make_dir(".cache/mysteel");
    let output_path = if cmd.output_path == "" {
        format!("common_data_{}_{}.csv", cmd.category_code, cmd.chart_code)
    } else {
        cmd.output_path.to_string()
    };
    let v = mysteel::get_common_data_list(&cmd.category_code, &cmd.chart_code).await;
    let output_path = format!(".cache/mysteel/{output_path}");
    let mut output = File::create(&output_path).expect("创建文件失败");
    let mut w = Vec::new();
    for r in &v {
        info!("r={:?}", r);
        write!(&mut w, "{},{}\n", r.data_date, r.data_value)?;
    }
    output.write_all(&w)?;
    info!("数据存放在 {output_path}");
    Ok(())
}
