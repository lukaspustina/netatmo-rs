use std::collections::HashMap;
use reqwest;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;

pub struct Settings<'a> {
    pub cliend_id: &'a str,
    pub client_secret: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> From<&'a Settings<'a>> for HashMap<&str, &'a str> {
    fn from(s: &'a Settings) -> HashMap<&'static str, &'a str> {
        let mut m = HashMap::new();
        m.insert("client_id", s.cliend_id);
        m.insert("client_secret", s.client_secret);
        m.insert("username", s.username);
        m.insert("password", s.password);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: Vec<Scope>,
    pub expires_in: u64,
    pub expire_in: u64,
}

pub fn get_token(s: &Settings, scopes: &[Scope]) -> Token {
    let scopes_str: String = scopes.into_iter().map(|s| s.to_scope_str()).collect::<Vec<_>>().as_slice().join(".");

    let mut params: HashMap<_,_> = s.into();
    params.insert("grant_type", "password");
    params.insert("scope", &scopes_str);

    let client = reqwest::Client::new();
    let mut res = client.post("https://api.netatmo.com/oauth2/token")
        .form(&params)
        .send().unwrap();

    let body = res.text().unwrap();
    let token: Token = serde_json::from_str(&body).unwrap();

    token
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    use std::collections::HashMap;

    mod settings {
        use super::*;
        #[test]
        fn into_hash_map() {
            let settings = Settings {
                cliend_id: "client_id",
                client_secret: "client_secret",
                username: "username",
                password: "password",
            };

            let s = &settings;
            let m: HashMap<_,_> = s.into();

            assert_that(&m).contains_key(&"client_id").is_equal_to(&"client_id");
            assert_that(&m).contains_key(&"client_secret").is_equal_to(&"client_secret");
            assert_that(&m).contains_key(&"username").is_equal_to(&"username");
            assert_that(&m).contains_key(&"password").is_equal_to(&"password");
        }
    }
}

