use crate::{client::AuthenticatedClient, errors::Result};

use serde::Deserialize;
use std::{collections::HashMap, fmt};

pub struct Parameters<'a> {
    home_id: &'a str,
    room_id: &'a str,
    mode: Mode,
    temp: Option<f32>,
    endtime: Option<usize>,
}

pub enum Mode {
    Manual,
    Home,
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Mode::Manual => "manual",
            Mode::Home => "home",
        };
        write!(f, "{}", s)
    }
}

impl<'a> Parameters<'a> {
    pub fn new(home_id: &'a str, room_id: &'a str, mode: Mode) -> Self {
        Parameters {
            home_id,
            room_id,
            mode,
            temp: None,
            endtime: None,
        }
    }

    pub fn temp(self, temp: f32) -> Self {
        Parameters {
            temp: Some(temp),
            ..self
        }
    }

    pub fn date_end(self, date_end: usize) -> Self {
        Parameters {
            endtime: Some(date_end),
            ..self
        }
    }
}

pub enum Type {
    Temperature,
    Humidity,
    CO2,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Type::Temperature => "Temperature",
            Type::Humidity => "Humidity",
            Type::CO2 => "CO2",
        };
        write!(f, "{}", s)
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> From<&'a Parameters<'a>> for HashMap<&str, String> {
    fn from(p: &'a Parameters) -> HashMap<&'static str, String> {
        let mut map = HashMap::default();
        map.insert("home_id", p.home_id.to_string());
        map.insert("room_id", p.room_id.to_string());
        map.insert("mode", p.mode.to_string());
        if let Some(temp) = p.temp {
            map.insert("temp", temp.to_string());
        }
        if let Some(endtime) = p.endtime {
            map.insert("endtime", endtime.to_string());
        }

        map
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: String,
    pub time_server: usize,
}

// cf. https://dev.netatmo.com/resources/technical/reference/energy/setroomthermpoint
pub fn set_room_thermpoint(client: &AuthenticatedClient, parameters: &Parameters) -> Result<Response> {
    let params: HashMap<&str, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (*k, v.as_ref())).collect();

    client.call("https://api.netatmo.com/api/setroomthermpoint", &mut params)
}
