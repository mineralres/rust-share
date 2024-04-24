use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use clap::Parser;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Default, Serialize, Deserialize)]
struct DingDingTextContent {
    pub content: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct DingDingMsgTarget {
    pub is_at_all: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DingDingSendMsgRequest {
    pub msgtype: String,
    pub text: DingDingTextContent,
    pub at: DingDingMsgTarget,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MailNotifyConfig {
    pub account: String,
    pub auth: String,
    pub smtp_server: String,
    pub subscribers: Vec<String>,
}

pub async fn send_dingding_text_msg(
    msg: &str,
    key_word: &str,
    access_token: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("https://oapi.dingtalk.com/robot/send?access_token={access_token}");
    let client = reqwest::Client::new();
    let mut req = DingDingSendMsgRequest::default();
    req.msgtype = "text".to_string();
    req.text.content = format!("{}: {}", key_word, msg);
    let body = serde_json::to_string(&req).unwrap();
    let _buf = client
        .post(&url)
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

pub fn send_email(
    account: &str,
    auth: &str,
    smtp_server: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.as_bytes().to_vec())
        .expect("");
    let creds = Credentials::new(account.to_string(), auth.to_string());
    let mailer = SmtpTransport::starttls_relay(smtp_server)?
        .credentials(creds)
        .build();

    // Send the email
    let _resp = mailer.send(&email)?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PumpConfig {
    pub email_config: MailNotifyConfig,
    pub dd_access_token: String,
    pub dd_key_word: String,
    pub http_port: i32,
    pub token: String,
}

impl PumpConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ReqLogInfo {
    pub title: String,
    pub content: String,
    pub token: String,
}

async fn log_info(State(s): State<MyState>, Json(req): Json<ReqLogInfo>) -> StatusCode {
    let email_config = &s.config.email_config;
    for s in &email_config.subscribers {
        match send_email(
            &email_config.account,
            &email_config.auth,
            &email_config.smtp_server,
            &email_config.account,
            &s,
            &req.title,
            &req.content,
        ) {
            Ok(_v) => {}
            Err(e) => error!("发送给[{}]通知邮件失败{}", s, e),
        }
    }
    match send_dingding_text_msg(
        &req.content,
        &s.config.dd_key_word,
        &s.config.dd_access_token,
    )
    .await
    {
        Ok(_) => (),
        Err(e) => error!("发送钉钉通知失败={:?}", e),
    }
    info!("{:?}", req);
    StatusCode::OK
}

#[derive(Clone)]
pub struct MyState {
    config: PumpConfig,
}

#[tokio::main]
async fn main() {
    base::init_logger();
    let opts: Opts = Opts::parse();
    let config = PumpConfig::load(&opts.config).expect("");
    info!("config={:?}", config);
    info!("日志泵启动 http_port:{}", config.http_port);
    let state = MyState {
        config: config.clone(),
    };
    let app = Router::new()
        .route("/log/info", post(log_info))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.http_port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
