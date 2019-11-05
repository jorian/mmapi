use reqwest::Client as HttpClient;
use std::fmt::Debug;
use std::io::Read;

use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::error::ApiError;

pub struct Client {
    client: HttpClient,
    url: String
}

#[derive(Debug)]
pub enum RpcClientError {
    Transport(reqwest::Error),
    Json(serde_json::Error),
}

impl Client {
    pub(crate) fn new() -> Self {
        let http_client = HttpClient::new();

        Client {
            client: http_client,
            url: "http://127.0.0.1:7783".to_string(),
        }
    }

    pub(crate) fn send<R, T>(
        &self,
        request: T,
    ) -> Result<R, ApiError>
        where
            T: Serialize + Debug,
            R: DeserializeOwned + Debug,
    {
        let res = self
            .client
            .post(self.url.as_str())
            .json(&request)
            .send()
            .map_err(|err| RpcClientError::Transport(err))
            .and_then(|mut res| {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf);
                dbg!(&buf);

                serde_json::from_str(&buf).map_err(|err| RpcClientError::Json(err))
            });

        dbg!(&res);

        let res = res.map(RpcResponse::into_result);

        match res {
            Ok(result) => {
                match result {
                    Ok(result) => Ok(result),
                    Err(rpc_error) => Err(ApiError::RPC(rpc_error))
                }
            },
            Err(client_error) => Err(ApiError::Client(client_error))
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RpcResponse<R> {
    pub result: Option<R>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RpcError {
    pub message: String
}

impl<R> RpcResponse<R> {
    pub fn into_result(self) -> Result<R, RpcError> {
        match self {
            RpcResponse {
                error: Some(rpc_error),
                result: None,
                ..
            } => Err(rpc_error),
//            RpcResponse {
//                error: None,
//                result: None,
//                ..
//            } => Ok(self),
            RpcResponse {
                error: None,
                result: Some(result),
                ..
            } => Ok(result),
            _ => unreachable!()
        }
    }
}
