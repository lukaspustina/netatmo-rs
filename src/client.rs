pub mod authenticate;
pub mod get_station_data;

pub use crate::errors::{Error, ErrorKind, Result};
pub use authenticate::{Scope, Token};
pub use get_station_data::StationData;

use failure::Fail;
use reqwest;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait Netatmo {
    fn get_station_data(&self, device_id: &str) -> Result<StationData>;
}

#[derive(Debug)]
pub struct ClientCredentials<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

pub struct NetatmoClient {}

impl<'a> NetatmoClient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(client_credentials: &'a ClientCredentials) -> UnauthenticatedClient<'a> {
        UnauthenticatedClient {
            client_credentials,
            http: reqwest::Client::new(),
        }
    }

    pub fn with_token(token: Token) -> AuthenticatedClient {
        AuthenticatedClient {
            token,
            http: reqwest::Client::new(),
        }
    }
}

#[derive(Debug)]
pub struct UnauthenticatedClient<'a> {
    client_credentials: &'a ClientCredentials<'a>,
    http: reqwest::Client,
}

impl<'a> UnauthenticatedClient<'a> {
    pub fn authenticate(
        self,
        username: &'a str,
        password: &'a str,
        scopes: &[Scope],
    ) -> Result<AuthenticatedClient> {
        authenticate::get_token(&self, username, password, scopes)
            .map(|token| AuthenticatedClient {
                token,
                http: self.http,
            })
            .map_err(|e| e.context(ErrorKind::AuthenticationFailed).into())
    }

    pub(crate) fn call<T>(&self, url: &str, params: &HashMap<&str, &str>) -> Result<T>
        where T: DeserializeOwned {
        api_call(&self.http, url, params)
    }
}

pub struct AuthenticatedClient {
    token: Token,
    http: reqwest::Client,
}

impl AuthenticatedClient {
    pub fn token(&self) -> &Token {
        &self.token
    }

    pub(crate) fn call<T>(&self, url: &str, params: &HashMap<&str, &str>) -> Result<T>
        where T: DeserializeOwned {
        api_call(&self.http, url, params)
    }
}

fn api_call<T>(http: &reqwest::Client, url: &str, params: &HashMap<&str, &str>) -> Result<T>
    where
        T: DeserializeOwned {
    let mut res = http
        .post(url)
        .form(&params)
        .send()
        .map_err(|e| e.context(ErrorKind::FailedToSendRequest))?;

    let body = res
        .text()
        .map_err(|e| e.context(ErrorKind::FailedToReadResponse))?;
    serde_json::from_str::<T>(&body).map_err(|e| e.context(ErrorKind::JsonDeserializationFailed).into())
}

impl Netatmo for AuthenticatedClient {
    fn get_station_data(&self, device_id: &str) -> Result<StationData> {
        get_station_data::get_station_data(&self, device_id).map_err(|e| {
            e.context(ErrorKind::ApiCallFailed("get_station_data".to_string()))
                .into()
        })
    }
}
