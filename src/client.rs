pub mod authenticate;
pub mod get_station_data;

pub use authenticate::{Scope, Token};
pub use get_station_data::StationData;
pub use crate::errors::{Error, ErrorKind, Result};

use failure::Fail;

pub trait Netatmo {
    fn get_station_data(&self, device_id: &DeviceId) -> Result<StationData>;
}

type DeviceId = String;

#[derive(Debug)]
pub struct ClientCredentials<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

pub struct NetatmoClient {}

impl<'a> NetatmoClient {
    pub fn new(client_credentials: &'a ClientCredentials) -> UnauthenticatedClient<'a> {
        UnauthenticatedClient {
            client_credentials,
        }
    }

    pub fn with_token(token: Token) -> AuthenticatedClient {
        AuthenticatedClient {
            token
        }

    }
}

#[derive(Debug)]
pub struct UnauthenticatedClient<'a> {
    client_credentials: &'a ClientCredentials<'a>
}

impl<'a> UnauthenticatedClient<'a> {
    pub fn authenticate(&self, username: &'a str, password: &'a str, scopes: &[Scope]) -> Result<AuthenticatedClient> {
        authenticate::get_token(self, username, password, scopes)
            .map(|token| AuthenticatedClient { token })
            .map_err(|e| e.context(ErrorKind::AuthenticationFailed).into())
    }
}

pub struct AuthenticatedClient {
    token: Token
}

impl AuthenticatedClient {
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Netatmo for AuthenticatedClient {
    fn get_station_data(&self, device_id: &DeviceId) -> Result<StationData> {
        get_station_data::get_station_data(&self.token, device_id)
            .map_err(|e| e.context(ErrorKind::ApiCallFailed("get_station_data".to_string())).into())
    }
}