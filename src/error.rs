use crate::network::client::{RpcError, RpcClientError};
use core::fmt;
use serde::export::Formatter;
use serde::export::fmt::Error;

// all the errors that this api can throw
#[derive(Debug)]
pub enum ApiError {
    RPC(RpcError),
    Client(RpcClientError)
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ApiError::Client(ref cause) =>
            match cause {
                RpcClientError::Json(e) => fmt::Display::fmt(e, f),
                RpcClientError::Transport(transport_error) => fmt::Display::fmt(transport_error, f),
            },
            ApiError::RPC(ref cause) => write!(f, "RPC error: {}", cause.message),
        }
    }
}