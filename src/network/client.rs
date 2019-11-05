use reqwest::Client as HttpClient;
use std::fmt::Debug;
use std::io::Read;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Client {
    client: HttpClient,
    url: String
}

#[derive(Debug)]
pub enum ClientError {
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
    ) -> R
        where
            T: Serialize + Debug,
            R: DeserializeOwned + Debug,
    {
        let res = self
            .client
            .post(self.url.as_str())
            .json(&request)
            .send()
            .map_err(|err| ClientError::Transport(err))
            .and_then(|mut res| {
                let mut buf = String::new();
                let _ = res.read_to_string(&mut buf);
                dbg!(&buf);

                serde_json::from_str(&buf).map_err(|err| ClientError::Json(err))
            }); // todo gracefully handle an error

        dbg!(&res);

        match res {
            Ok(result) => result,
            Err(e) => {
                println!("Error: {:?}", e);
                panic!("nope");
            }
        }
    }
}