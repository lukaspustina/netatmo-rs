use crate::{client::UnauthenticatedClient, errors::Result};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token:  String,
    pub refresh_token: String,
    pub scope:         Vec<Scope>,
    pub expires_in:    u64,
    pub expire_in:     u64,
}

#[allow(clippy::implicit_hasher)]
impl<'a> From<&'a UnauthenticatedClient<'a>> for HashMap<&str, &'a str> {
    fn from(uc: &'a UnauthenticatedClient) -> HashMap<&'static str, &'a str> {
        let mut m = HashMap::default();
        m.insert("client_id", uc.client_credentials.client_id);
        m.insert("client_secret", uc.client_credentials.client_secret);

        m
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    ReadStation,
    ReadThermostat,
    WriteThermostat,
    ReadCamera,
    WriteCamera,
    AccessCamera,
    ReadPresence,
    AccessPresence,
    ReadHomecoach,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Scope::ReadStation => "ReadStation",
            Scope::ReadThermostat => "ReadThermostat",
            Scope::WriteThermostat => "WriteThermostat",
            Scope::ReadCamera => "ReadCamera",
            Scope::WriteCamera => "WriteCamera",
            Scope::AccessCamera => "AccessCamera",
            Scope::ReadPresence => "ReadPresence",
            Scope::AccessPresence => "AccessPresence",
            Scope::ReadHomecoach => "ReadHomecoach",
        };
        write!(f, "{}", s)
    }
}

impl Scope {
    fn to_scope_str(&self) -> &'static str {
        match self {
            Scope::ReadStation => "read_station",
            Scope::ReadThermostat => "read_thermostat",
            Scope::WriteThermostat => "write_thermostat",
            Scope::ReadCamera => "read_camera",
            Scope::WriteCamera => "write_camera",
            Scope::AccessCamera => "access_camera",
            Scope::ReadPresence => "read_presence",
            Scope::AccessPresence => "access_presence",
            Scope::ReadHomecoach => "read_homecoach",
        }
    }
}

pub(crate) fn get_token(
    unauthenticated_client: &UnauthenticatedClient,
    username: &str,
    password: &str,
    scopes: &[Scope],
) -> Result<Token> {
    let scopes_str: String = scopes
        .iter()
        .map(Scope::to_scope_str)
        .collect::<Vec<_>>()
        .as_slice()
        .join(".");

    let mut params: HashMap<_, _> = unauthenticated_client.into();
    params.insert("username", username);
    params.insert("password", password);
    params.insert("grant_type", "password");
    params.insert("scope", &scopes_str);

    unauthenticated_client.call("https://api.netatmo.com/oauth2/token", &params)
}
