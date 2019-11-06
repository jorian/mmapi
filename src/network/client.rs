use reqwest::Client as HttpClient;
use std::fmt::Debug;
use std::io::Read;

use serde::Serialize;
use serde::de::DeserializeOwned;
use crate::error::ApiError;
use serde_json::Value;

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

    // this send is used when the expected response contains:
    // - no result and no error key
    // - a result with just a "success" string.
    // in this case we don't need to further sanitize the result and we can just return the deserialized JSON.
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
            // if it already returns an error after sending, we know it happened in transport:
            .map_err(|err| ApiError::Client(RpcClientError::Transport(err)))
            // now we're going to work with JSON, so any error will be a JSON conversion error
            // a successful result is a Value we can work with.
            .and_then(|mut res| {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf);
                dbg!(&buf);

                // error is thrown when converting from string to json goes wrong.
                serde_json::from_str(&buf).map_err(|err| ApiError::Client(RpcClientError::Json(err)))
            });

        // this can still be an error. an error response is in the style:
        // {
        //      error: String
        // }
        // while a success response is:
        // {
        //      result: "success"
        //      ..
        // }
        // where in some cases the result key is optional.

        res
    }

    // this send is used when the actual result is nested in its own result field,
    // or when the atomicdex API returns an error.
    pub(crate) fn send2<R, T>(
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
            // if it already returns an error after sending, we know it happened in transport:
            .map_err(|err| ApiError::Client(RpcClientError::Transport(err)))
            // now we're going to work with JSON, so any error will be a JSON conversion error
            // a successful result is a Value we can work with.
            .and_then(|mut res| {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf);
                dbg!(&buf);

                // error is thrown when converting from string to json goes wrong.
                serde_json::from_str(&buf).map_err(|err| ApiError::Client(RpcClientError::Json(err)))
            });

        // if the result key contains an object or array, unwrap and return that.
        let res = res.map(RpcResponse::into_result);

        match res {
            Ok(result) => {
                match result {
                    Ok(result) => Ok(result),
                    Err(rpc_error) => Err(ApiError::RPC(rpc_error))
                }
            },
            Err(client_error) => Err(client_error)
        }
    }
}


// this part is just to differentiate the result into actual result and error.
// but, this doesn't work for our case, since the value of the result key can be a string, and if that
// is the case, the object that result is part of, is the object to use.
// also, if both result and error are not in the keyset, the object to return is also the object itself
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
            RpcResponse {
                error: None,
                result: Some(result),
                ..
            } => Ok(result),
            RpcResponse {
                error: None,
                result: None,
                ..
            } => panic!("no error and no result!"),
            _ => unreachable!()
        }
    }
}
