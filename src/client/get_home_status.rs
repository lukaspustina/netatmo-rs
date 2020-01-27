use crate::client::AuthenticatedClient;
use crate::errors::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::get_homes_data::GatewayType;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeStatus {
    pub status: String,
    pub time_server: i64,
    pub body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Body {
    pub home: Home,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub id: String,
    pub modules: Vec<Module>,
    pub rooms: Vec<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub firmware_revision: i64,
    pub rf_strength: Option<i64>,
    pub wifi_strength: Option<i64>,
    pub reachable: Option<bool>,
    pub battery_level: Option<i64>,
    pub boiler_valve_comfort_boost: Option<bool>,
    pub boiler_status: Option<bool>,
    pub anticipating: Option<bool>,
    pub bridge: Option<String>,
    pub battery_state: Option<String>,
    pub status_active: Option<bool>,
    pub status_tampered: Option<bool>,
    pub test_mode: Option<bool>,
    pub hush_mode: Option<bool>,
    pub smoke_detected: Option<bool>,
    pub detection_chamber_status: Option<String>,
    pub battery_alarm_state: Option<String>,
    pub battery_percent: Option<i64>,
    pub wifi_status: Option<i64>,
    pub last_smoke_detected_start_time: Option<i64>,
    pub last_smoke_detected_end_time: Option<i64>,
    pub last_seen: Option<i64>,
    pub last_wifi_connection: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub reachable: bool,
    pub therm_measured_temperature: f64,
    pub heating_power_request: i64,
    pub therm_setpoint_temperature: f64,
    pub therm_setpoint_mode: String,
    pub therm_setpoint_start_time: i64,
    pub therm_setpoint_end_time: i64,
    pub anticipating: bool,
    pub open_window: bool,
}

#[derive(Default)]
pub struct Parameters<'a> {
    home_id: Option<&'a str>,
    device_types: Option<&'a [GatewayType]>,
}

impl<'a> Parameters<'a> {
    pub fn new() -> Self {
        Parameters::default()
    }

    pub fn home_id(self, home_id: &'a str) -> Self {
        Parameters {
            home_id: Some(home_id),
            ..self
        }
    }
    pub fn device_types(self, device_types: &'a [GatewayType]) -> Self {
        Parameters {
            device_types: Some(device_types),
            ..self
        }
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> From<&'a Parameters<'a>> for HashMap<&str, String> {
    fn from(p: &'a Parameters) -> HashMap<&'static str, String> {
        let mut map = HashMap::default();
        if let Some(home_id) = p.home_id {
            map.insert("home_id", home_id.to_string());
        }
        if let Some(device_types) = p.device_types {
            let device_types = device_types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .as_slice()
                .join(",");
            map.insert("device_types", device_types);
        }

        map
    }
}

pub(crate) fn get_home_status(
    client: &AuthenticatedClient,
    parameters: &Parameters,
) -> Result<HomeStatus> {
    let params: HashMap<&str, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (*k, v.as_ref())).collect();
    client.call("https://api.netatmo.com/api/homestatus", &mut params)
}
