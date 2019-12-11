use crate::client::AuthenticatedClient;
use crate::errors::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub use get_homes_data::GatewayType;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HomeStatus {
    status: String,
    time_server: i64,
    body: Body,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Body {
    home: Home,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Home {
    id: String,
    modules: Vec<Module>,
    rooms: Vec<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Module {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    firmware_revision: i64,
    rf_strength: Option<i64>,
    wifi_strength: Option<i64>,
    reachable: Option<bool>,
    battery_level: Option<i64>,
    boiler_valve_comfort_boost: Option<bool>,
    boiler_status: Option<bool>,
    anticipating: Option<bool>,
    bridge: Option<String>,
    battery_state: Option<String>,
    status_active: Option<bool>,
    status_tampered: Option<bool>,
    test_mode: Option<bool>,
    hush_mode: Option<bool>,
    smoke_detected: Option<bool>,
    detection_chamber_status: Option<String>,
    battery_alarm_state: Option<String>,
    battery_percent: Option<i64>,
    wifi_status: Option<i64>,
    last_smoke_detected_start_time: Option<i64>,
    last_smoke_detected_end_time: Option<i64>,
    last_seen: Option<i64>,
    last_wifi_connection: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Room {
    id: String,
    reachable: bool,
    therm_measured_temperature: f64,
    heating_power_request: i64,
    therm_setpoint_temperature: f64,
    therm_setpoint_mode: String,
    therm_setpoint_start_time: i64,
    therm_setpoint_end_time: i64,
    anticipating: bool,
    open_window: bool,
}

#[derive(Default)]
pub struct Parameters<'a> {
    home_id: Option<&'a str>,
    gateway_types: Option<&'a [GatewayType]>,
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
    pub fn gateway_types(self, gateway_types: &'a [GatewayType]) -> Self {
        Parameters {
            gateway_types: Some(gateway_types),
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
        if let Some(gateway_types) = p.gateway_types {
            let gateway_types = gateway_types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .as_slice()
                .join(",");
            map.insert("gateway_types", gateway_types);
        }

        map
    }
}

pub(crate) fn get_home_status(
    client: &AuthenticatedClient,
    parameters: &Parameters,
) -> Result<HomesData> {
    let params: HashMap<&str, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (*k, v.as_ref())).collect();
    client.call("https://api.netatmo.com/api/homestatus", &mut params)
}
