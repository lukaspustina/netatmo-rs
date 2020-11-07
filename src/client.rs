use std::collections::HashMap;

use failure::Fail;
use log::trace;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;

use authenticate::{Scope, Token};
use get_home_status::HomeStatus;
use get_homes_data::HomesData;
use get_measure::Measure;
use get_station_data::StationData;

use crate::errors::{Error, ErrorKind, Result};

pub mod authenticate;
pub mod get_home_status;
pub mod get_homes_data;
pub mod get_measure;
pub mod get_station_data;
pub mod set_room_thermpoint;

pub trait Netatmo {
    fn get_home_status(&self, parameters: &get_home_status::Parameters) -> Result<HomeStatus>;
    fn get_homes_data(&self, parameters: &get_homes_data::Parameters) -> Result<HomesData>;
    fn get_station_data(&self, device_id: &str) -> Result<StationData>;
    fn get_measure(&self, parameters: &get_measure::Parameters) -> Result<Measure>;
    fn set_room_thermpoint(
        &self,
        parameters: &set_room_thermpoint::Parameters,
    ) -> Result<set_room_thermpoint::Response>;
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
    pub fn authenticate(self, username: &'a str, password: &'a str, scopes: &[Scope]) -> Result<AuthenticatedClient> {
        authenticate::get_token(&self, username, password, scopes)
            .map(|token| AuthenticatedClient { token, http: self.http })
            .map_err(|e| e.context(ErrorKind::AuthenticationFailed).into())
    }

    pub(crate) fn call<T>(&self, name: &'static str, url: &str, params: &HashMap<&str, &str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        api_call(name, &self.http, url, params)
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

    pub(crate) fn call<'a, T>(&'a self, name: &'static str, url: &str, params: &mut HashMap<&str, &'a str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        params.insert("access_token", &self.token.access_token);
        api_call(name, &self.http, url, params)
    }
}

fn api_call<T>(name: &'static str, http: &reqwest::Client, url: &str, params: &HashMap<&str, &str>) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut res = http
        .post(url)
        .form(&params)
        .send()
        .map_err(|e| e.context(ErrorKind::FailedToSendRequest))?
        .general_err_handler(name, StatusCode::OK)?;

    let body = res.text().map_err(|e| e.context(ErrorKind::FailedToReadResponse))?;
    trace!("Sucessful ({:?}) repsone: '{}'", res.status(), body);
    serde_json::from_str::<T>(&body).map_err(|e| e.context(ErrorKind::JsonDeserializationFailed).into())
}

pub(crate) trait GeneralErrHandler {
    type T: std::marker::Sized;

    fn general_err_handler(self, name: &'static str, expected_status: StatusCode) -> Result<Self::T>;
}

#[derive(Debug, Deserialize)]
struct ApiError {
    #[serde(rename = "error")]
    details: ApiErrorDetails,
}

#[derive(Debug, Deserialize)]
struct ApiErrorDetails {
    code: isize,
    message: String,
}

impl GeneralErrHandler for Response {
    type T = Response;

    fn general_err_handler(mut self, name: &'static str, expected_status: StatusCode) -> Result<Self> {
        match self.status() {
            code if code == expected_status => Ok(self),
            code @ StatusCode::BAD_REQUEST
            | code @ StatusCode::UNAUTHORIZED
            | code @ StatusCode::FORBIDDEN
            | code @ StatusCode::NOT_FOUND
            | code @ StatusCode::NOT_ACCEPTABLE
            | code @ StatusCode::INTERNAL_SERVER_ERROR => {
                let body = self.text().map_err(|e| {
                    e.context(ErrorKind::UnknownApiCallFailure {
                        name,
                        status_code: code.as_u16(),
                    })
                })?;
                let err: ApiError = serde_json::from_str(&body).map_err(|e| {
                    e.context(ErrorKind::UnknownApiCallFailure {
                        name,
                        status_code: code.as_u16(),
                    })
                })?;
                Err(Error::from(ErrorKind::ApiCallFailed {
                    name,
                    code: err.details.code,
                    msg: err.details.message,
                }))
            }
            code => Err(Error::from(ErrorKind::UnknownApiCallFailure {
                name,
                status_code: code.as_u16(),
            })),
        }
    }
}

impl Netatmo for AuthenticatedClient {
    fn get_homes_data(&self, parameters: &get_homes_data::Parameters) -> Result<HomesData> {
        get_homes_data::get_homes_data(&self, parameters)
    }

    fn get_home_status(&self, parameters: &get_home_status::Parameters) -> Result<HomeStatus> {
        get_home_status::get_home_status(&self, parameters)
    }

    fn get_station_data(&self, device_id: &str) -> Result<StationData> {
        get_station_data::get_station_data(&self, device_id)
    }

    fn get_measure(&self, parameters: &get_measure::Parameters) -> Result<Measure> {
        get_measure::get_measure(&self, parameters)
    }

    fn set_room_thermpoint(
        &self,
        parameters: &set_room_thermpoint::Parameters,
    ) -> Result<set_room_thermpoint::Response> {
        set_room_thermpoint::set_room_thermpoint(&self, parameters)
    }
}
