pub mod http {
    use crate::executor::Executor;
    use crate::{error::Error, executor::ReqMessage};
    use axum::{
        extract::{Json, State},
        response::Response,
        routing::any,
        Router,
    };
    use futures::StreamExt;
    use log::info;
    use ::ctp_futures::*;
    use ::rust_share_util::*;
    use std::ffi::{CStr, CString};
    use std::io::Write;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use types::*;

    impl axum::response::IntoResponse for Error {
        fn into_response(self) -> Response {
            XResponse {
                data: "".to_string(),
                msg: self.to_string(),
                code: 0,
            }
            .into_response()
        }
    }

    mod types {
        use std::sync::Arc;

        use axum::{extract::Json, response::Response};
        use serde::Serialize;

        use crate::executor::{ContractPositionTarget, Executor};
        use tokio::sync::Mutex;

        #[derive(Clone)]
        pub struct ShareState {
            pub config: crate::Config,
            pub executor: Arc<Mutex<Executor>>,
        }

        #[derive(Debug, Serialize)]
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

        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct ReqQuery {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct ReqQueryTradingAccount {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct ReqSetContractTarget {
            pub broker_id: String,
            pub account: String,
            pub target: ContractPositionTarget,
        }
    }

    pub async fn serve(conf: crate::Config, executor: Arc<Mutex<Executor>>) {
        let addr = SocketAddr::from(([0, 0, 0, 0], conf.http_port));
        info!("Http listening on {}", addr);
        let state = ShareState {
            config: conf,
            executor,
        };
        let app = Router::new()
            .route("/api/query_position_detail", any(query_position_detail))
            .route("/api/query_trading_account", any(query_trading_account))
            .route("/api/set_contract_target", any(set_contract_target))
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
    ) -> Result<Vec<u8>, Error> {
        let req_msg = ReqMessage::QueryTradingAccount;
        let resp = s
            .executor
            .lock()
            .await
            .query(&req.account, req_msg)
            .await??;
        let config = bincode::config::standard();
        let encoded: Vec<u8> = bincode::encode_to_vec(&resp, config).unwrap();
        Ok(encoded)
    }

    async fn query_position_detail(
        State(s): State<ShareState>,
        Json(req): Json<ReqQuery>,
    ) -> Result<(), Error> {
        let req_msg = ReqMessage::QueryPositionDetail;
        let resp = s.executor.lock().await.query(&req.account, req_msg).await?;
        info!("resp={:?}", resp);
        Ok(())
    }

    async fn set_contract_target(
        State(s): State<ShareState>,
        Json(req): Json<ReqSetContractTarget>,
    ) -> Result<(), Error> {
        if req.target.symbol == "" {
            return Err(Error::InvalidSymbol);
        }
        let req_msg = ReqMessage::SetContractTarget(req.target);
        let resp = s
            .executor
            .lock()
            .await
            .query(&req.account, req_msg)
            .await??;
        info!("resp={:?}", resp);
        Ok(())
    }
}
