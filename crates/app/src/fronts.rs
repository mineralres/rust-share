pub mod http {
    use crate::executor::Executor;
    use axum::{
        extract::{Json, State},
        response::Response,
        routing::any,
        Router,
    };
    use base::ReqMessage;
    use log::info;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use types::*;

    #[derive(Debug, derive_more::Display, derive_more::From)]
    pub enum Error {
        BaseErr(base::error::Error),
        SimpleErr(simple_error::SimpleError),
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
        use std::sync::Arc;

        use axum::{extract::Json, response::Response};
        use serde::Serialize;

        use crate::executor::Executor;
        use base::state::ContractPositionTarget;
        use tokio::sync::Mutex;

        #[derive(Clone)]
        pub struct ShareState {
            pub config: base::ExecutorConfig,
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

        #[derive(Default, serde::Serialize, serde::Deserialize, std::fmt::Debug)]
        pub struct ReqQuery {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize)]
        pub struct ReqQueryTradingAccount {
            pub broker_id: String,
            pub account: String,
        }

        #[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
        pub struct ReqSetContractTarget {
            pub broker_id: String,
            pub account: String,
            pub target: ContractPositionTarget,
        }
    }

    pub async fn serve(conf: base::ExecutorConfig, executor: Arc<Mutex<Executor>>) {
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
        Ok(resp)
    }

    async fn query_position_detail(
        State(s): State<ShareState>,
        Json(req): Json<ReqQuery>,
    ) -> Result<Vec<u8>, Error> {
        let req_msg = ReqMessage::QueryPositionDetail;
        info!("query_position_detail={:?}", req);
        let resp = s
            .executor
            .lock()
            .await
            .query(&req.account, req_msg)
            .await??;
        info!("query_position_detail resp={:?}", resp.len());
        Ok(resp)
    }

    async fn set_contract_target(
        State(s): State<ShareState>,
        Json(req): Json<ReqSetContractTarget>,
    ) -> Result<(), Error> {
        info!(
            "set_contract_target [{}:{}] position={} shift={}",
            req.target.exchange, req.target.symbol, req.target.position, req.target.shift
        );
        if req.target.symbol == "" {
            return Err(Error::SimpleErr(simple_error::SimpleError::new(
                "Symbol can't be null",
            )));
        }
        let req_msg = ReqMessage::SetContractTarget(req.target);
        let _resp = s
            .executor
            .lock()
            .await
            .query(&req.account, req_msg)
            .await??;
        Ok(())
    }
}
