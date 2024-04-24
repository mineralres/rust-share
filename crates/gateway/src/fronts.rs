pub mod http {
    use crate::executor::Executor;
    use axum::{
        extract::{Json, State},
        response::{IntoResponse, Response},
        routing::any,
        Router,
    };
    use base::state::ReqMessage;
    use log::{error, info, warn};
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use types::*;

    #[derive(Debug, derive_more::Display, derive_more::From)]
    pub enum Error {
        BaseErr(base::error::Error),
        SimpleErr(simple_error::SimpleError),
        Base64DocodeErr(base64::DecodeError),
        TraderOcupied,
        TraderIsNotLocked,
        InvalidToken,
        TraderNotFound,
        CredentialExpired,
        AcquireLockFirst,
    }

    impl axum::response::IntoResponse for Error {
        fn into_response(self) -> Response {
            XResponse {
                data: "".to_string(),
                msg: self.to_string(),
                code: -1,
            }
            .into_response()
        }
    }

    pub mod types {
        use super::Executor;
        use axum::{extract::Json, response::Response};
        use base::{state::ContractPositionTarget, TradingAccountConfig};
        use serde::{Deserialize, Serialize};
        use std::sync::Arc;
        use tokio::sync::Mutex;

        #[derive(Clone)]
        pub struct LockedItem {
            pub broker_id: String,
            pub account: String,
            pub credential: String,
            pub timestamp: i64,
        }

        #[derive(Clone)]
        pub struct ShareState {
            pub config: base::ExecutorConfig,
            pub executor: Arc<Mutex<Executor>>,
            pub locked_client: Arc<Mutex<Vec<LockedItem>>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct XResponse<T> {
            pub data: T,
            pub msg: String,
            pub code: i32,
        }

        impl<T: serde::Serialize> axum::response::IntoResponse for XResponse<T> {
            fn into_response(self) -> Response {
                Json(self).into_response()
            }
        }

        impl<T> XResponse<T> {
            pub fn new(data: T) -> Self {
                Self {
                    code: 0,
                    msg: "".into(),
                    data,
                }
            }
        }

        impl From<&Vec<u8>> for XResponse<String> {
            fn from(value: &Vec<u8>) -> Self {
                use base64::prelude::*;
                let data = BASE64_STANDARD.encode(value);
                XResponse::<String> {
                    code: 0,
                    msg: "".into(),
                    data,
                }
            }
        }

        // impl XResponse<String> {
        //     pub fn new(v: &Vec<u8>) -> Self {
        //         use base64::prelude::*;
        //         let data = BASE64_STANDARD.encode(v);
        //         Self {
        //             code: 0,
        //             msg: "".into(),
        //             data,
        //         }
        //     }
        // }

        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct ReqQueryTradingAccount {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqQueryPositionDetail {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqSetContractTarget {
            pub broker_id: String,
            pub account: String,
            pub target: ContractPositionTarget,
            pub credential: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqFullQuery {
            pub ta: TradingAccountConfig,
            // pub timeout: i32,
        }
        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqExitProcess {
            pub delay: i32,
        }
        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqLockTradingAccount {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqRefreshLockTradingAccount {
            pub broker_id: String,
            pub account: String,
            pub credential: String,
        }
    }

    async fn check_token_middleware(
        State(s): State<ShareState>,
        req: axum::extract::Request,
        next: axum::middleware::Next,
    ) -> Response {
        match req.headers().get("token").map(|x| *x == s.config.token) {
            Some(true) => next.run(req).await,
            Some(false) | None => Error::InvalidToken.into_response(),
        }
    }

    pub async fn serve(conf: base::ExecutorConfig, executor: Arc<Mutex<Executor>>) {
        let addr = SocketAddr::from(([0, 0, 0, 0], conf.http_port));
        info!("Http listening on {}", addr);
        let state = ShareState {
            config: conf,
            executor,
            locked_client: Arc::new(Mutex::new(vec![])),
        };
        let app = Router::new()
            .route(
                "/api/refresh_lock_trading_account",
                any(refresh_lock_trading_account),
            )
            .route("/api/lock_trading_account", any(lock_trading_account))
            .route("/api/full_query", any(full_query))
            .route("/api/query_position_detail", any(query_position_detail))
            .route("/api/query_trading_account", any(query_trading_account))
            .route("/api/set_contract_target", any(set_contract_target))
            .route("/api/exit_process", any(exit_process))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                check_token_middleware,
            ))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("监听错误:");
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
    async fn query_trading_account(
        State(s): State<ShareState>,
        Json(req): Json<ReqQueryTradingAccount>,
    ) -> Result<XResponse<String>, Error> {
        let req_msg = ReqMessage::QueryTradingAccount;
        let resp = s
            .executor
            .lock()
            .await
            .query(&req.account, req_msg)
            .await??;
        Ok(XResponse::<String>::from(&resp))
    }

    async fn query_position_detail(
        State(s): State<ShareState>,
        Json(req): Json<ReqQueryPositionDetail>,
    ) -> Result<XResponse<String>, Error> {
        let req_msg = ReqMessage::QueryPositionDetail;
        info!("query_position_detail={:?}", req);
        let resp = s
            .executor
            .lock()
            .await
            .query(&req.account, req_msg)
            .await??;
        info!("query_position_detail resp={:?}", resp.len());
        Ok(XResponse::<String>::from(&resp))
    }

    async fn set_contract_target(
        State(s): State<ShareState>,
        Json(req): Json<ReqSetContractTarget>,
    ) -> Result<XResponse<String>, Error> {
        // info!(
        //     "set_contract_target [{}:{}] position={} shift={}",
        //     req.target.exchange, req.target.symbol, req.target.position, req.target.shift
        // );
        if req.target.symbol == "" {
            return Err(Error::SimpleErr(simple_error::SimpleError::new(
                "Symbol can't be null",
            )));
        }
        if s.config.lock_check {
            let v = s.locked_client.lock().await;
            let now = chrono::Local::now().timestamp();
            let credential_valid = v
                .iter()
                .find(|x| x.broker_id == req.broker_id && x.account == req.account)
                .map(|x| {
                    x.credential == req.credential
                        && now - x.timestamp < s.config.lock_credential_ttl
                })
                .unwrap_or(false);
            if !credential_valid {
                return Err(Error::TraderIsNotLocked);
            }
        }
        let target = req.target.clone();
        let req_msg = ReqMessage::SetContractTarget(req.target);
        let resp = s.executor.lock().await.query(&req.account, req_msg).await?;
        if let Err(e) = &resp {
            error!("set_contract_target {e} , target={:?}", target);
        }
        let resp = resp?;
        Ok(XResponse::<String>::from(&resp))
    }

    #[axum_macros::debug_handler]
    async fn full_query(
        State(_s): State<ShareState>,
        Json(req): Json<ReqFullQuery>,
    ) -> Result<XResponse<String>, Error> {
        let timeout = 50;
        match req.ta.route_type.as_str() {
            "ctp_futures" => {
                match tokio::time::timeout(
                    tokio::time::Duration::from_secs(timeout),
                    ctp_futures::query::query(&req.ta),
                )
                .await
                {
                    Ok(result) => {
                        let result = result?;
                        let config = bincode::config::standard();
                        let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
                        Ok(XResponse::<String>::from(&encoded))
                    }
                    Err(_) => Err(Error::BaseErr(base::error::Error::QueryTimeout)),
                }
            }
            "tora_stock" => {
                match tokio::time::timeout(
                    tokio::time::Duration::from_secs(timeout),
                    tora_stock::query::query(&req.ta),
                )
                .await
                {
                    Ok(result) => {
                        let result = result?;
                        let config = bincode::config::standard();
                        let encoded: Vec<u8> = bincode::encode_to_vec(&result, config).unwrap();
                        Ok(XResponse::<String>::from(&encoded))
                    }
                    Err(_) => Err(Error::BaseErr(base::error::Error::QueryTimeout)),
                }
            }
            _ => return Err(Error::BaseErr(base::error::Error::InvalidRouteType)),
        }
    }

    async fn exit_process(
        State(_s): State<ShareState>,
        Json(req): Json<ReqExitProcess>,
    ) -> Result<XResponse<()>, Error> {
        warn!("req exit process {:?}", req);
        if req.delay > 0 {
            let delay = std::cmp::min(req.delay, 2000);
            tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(delay as u64)).await;
                log::warn!("Exit process after {delay} seconds!!!");
                std::process::exit(0);
            });
        } else {
            log::warn!("Exit process on request!!!");
            std::process::exit(0);
        }
        let xresp = XResponse {
            data: (),
            msg: "".into(),
            code: 0,
        };
        Ok(xresp)
    }

    fn gen_key() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        let mut rng = rand::thread_rng();
        let key: String = (0..30)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        key
    }

    async fn lock_trading_account(
        State(s): State<ShareState>,
        Json(req): Json<ReqLockTradingAccount>,
    ) -> Result<XResponse<String>, Error> {
        let mut v = s.locked_client.lock().await;
        let now = chrono::Local::now().timestamp();
        let is_locked = v
            .iter()
            .find(|x| x.broker_id == req.broker_id && x.account == req.account)
            .map(|x| x.credential.len() > 0 && now - x.timestamp < s.config.lock_credential_ttl)
            .unwrap_or(false);
        if is_locked {
            Err(Error::TraderOcupied)
        } else {
            let new_key = gen_key();
            match v
                .iter_mut()
                .find(|x| x.broker_id == req.broker_id && x.account == req.account)
            {
                Some(i) => {
                    i.credential = new_key.clone();
                    i.timestamp = now;
                }
                None => v.push(LockedItem {
                    broker_id: req.broker_id.clone(),
                    account: req.account.clone(),
                    credential: new_key.clone(),
                    timestamp: now,
                }),
            }
            Ok(XResponse::new(new_key))
        }
    }

    async fn refresh_lock_trading_account(
        State(s): State<ShareState>,
        Json(req): Json<ReqRefreshLockTradingAccount>,
    ) -> Result<XResponse<String>, Error> {
        let mut v = s.locked_client.lock().await;
        let now = chrono::Local::now().timestamp();
        match v.iter_mut().find(|x| {
            x.broker_id == req.broker_id
                && x.account == req.account
                && req.credential == req.credential
        }) {
            Some(i) => {
                if now - i.timestamp > s.config.lock_credential_ttl {
                    Err(Error::CredentialExpired)
                } else {
                    if req.credential == i.credential {
                        i.timestamp = now;
                        Ok(XResponse::new("".into()))
                    } else {
                        Err(Error::TraderOcupied)
                    }
                }
            }
            None => Err(Error::AcquireLockFirst),
        }
    }
}
