use crate::client::AuthenticatedClient;
use crate::errors::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StationData {
    body: Body,
    status: String,
    time_exec: f64,
    time_server: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    devices: Vec<Device>,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id")]
    id: String,
    cipher_id: String,
    co2_calibrating: bool,
    date_setup: u64,
    firmware: u64,
    last_setup: u64,
    last_status_store: u64,
    last_upgrade: u64,
    module_name: String,
    reachable: bool,
    station_name: String,
    #[serde(rename = "type")]
    type_info: String,
    wifi_status: f64,
    dashboard_data: DashboardData,
    data_type: Vec<String>,
    modules: Vec<Module>,
    place: Place,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    #[serde(rename = "AbsolutePressure")]
    absolute_pressure: Option<f64>,
    #[serde(rename = "CO2")]
    co2: Option<u64>,
    #[serde(rename = "Humidity")]
    humidity: Option<u64>,
    #[serde(rename = "Noise")]
    noise: Option<u64>,
    #[serde(rename = "Pressure")]
    pressure: Option<f64>,
    #[serde(rename = "Temperature")]
    temperature: Option<f64>,
    date_max_temp: Option<u64>,
    date_min_temp: Option<u64>,
    max_temp: Option<f64>,
    min_temp: Option<f64>,
    pressure_trend: Option<String>,
    temp_trend: Option<String>,
    time_utc: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    #[serde(rename = "_id")]
    id: String,
    battery_percent: u64,
    battery_vp: u64,
    dashboard_data: DashboardData,
    data_type: Vec<String>,
    firmware: u64,
    last_message: u64,
    last_seen: u64,
    last_setup: u64,
    module_name: String,
    reachable: bool,
    rf_status: u64,
    #[serde(rename = "type")]
    type_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    altitude: u64,
    city: String,
    country: String,
    location: Vec<f64>,
    timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    administrative: Administrative,
    mail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Administrative {
    feel_like_algo: u64,
    lang: String,
    pressureunit: u64,
    reg_locale: String,
    unit: u64,
    windunit: u64,
}

pub(crate) fn get_station_data(client: &AuthenticatedClient, device_id: &str) -> Result<StationData> {
    let mut params: HashMap<&str, &str> = HashMap::default();
    params.insert("access_token", &client.token.access_token);
    params.insert("device_id", device_id);

    client.call("https://api.netatmo.com/api/getstationsdata", &params)
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

    use std::collections::HashMap;

    mod get_station_data {
        use super::*;

        #[test]
        fn parse_response() {
            let json = r#"{
  "body": {
    "devices": [
      {
        "_id": "12:34:56:78:90:AB",
        "cipher_id": "enc:16:icj48gjlkt399g+dkkdklj490999 lkfkjfgjkjklk3440fjjj300cxq2399dkdd",
        "co2_calibrating": false,
        "dashboard_data": {
          "AbsolutePressure": 1013.3,
          "CO2": 455,
          "Humidity": 43,
          "Noise": 40,
          "Pressure": 1019.3,
          "Temperature": 20.3,
          "date_max_temp": 1556437566,
          "date_min_temp": 1556448808,
          "max_temp": 22.3,
          "min_temp": 20.2,
          "pressure_trend": "up",
          "temp_trend": "stable",
          "time_utc": 1556451224
        },
        "data_type": [
          "Temperature",
          "CO2",
          "Humidity",
          "Noise",
          "Pressure"
        ],
        "date_setup": 1556295333,
        "firmware": 140,
        "last_setup": 1556295333,
        "last_status_store": 1556451233,
        "last_upgrade": 1556295520,
        "module_name": "Inside",
        "modules": [
          {
            "_id": "12:34:56:78:90:CD",
            "battery_percent": 100,
            "battery_vp": 6190,
            "dashboard_data": {
              "Humidity": 53,
              "Temperature": 13.8,
              "date_max_temp": 1556450543,
              "date_min_temp": 1556425125,
              "max_temp": 13.8,
              "min_temp": 10,
              "temp_trend": "up",
              "time_utc": 1556451208
            },
            "data_type": [
              "Temperature",
              "Humidity"
            ],
            "firmware": 46,
            "last_message": 1556451228,
            "last_seen": 1556451208,
            "last_setup": 1556295333,
            "module_name": "Outside",
            "reachable": true,
            "rf_status": 86,
            "type": "NAModule1"
          }
        ],
        "place": {
          "altitude": 50,
          "city": "Alert",
          "country": "CAN",
          "location": [
            82.5057837,
            -62.5575262
          ],
          "timezone": "EDT"
        },
        "reachable": true,
        "station_name": "Home",
        "type": "NAMain",
        "wifi_status": 50
      }
    ],
    "user": {
      "administrative": {
        "feel_like_algo": 0,
        "lang": "en-US",
        "pressureunit": 0,
        "reg_locale": "en-US",
        "unit": 0,
        "windunit": 0
      },
      "mail": "lukas at my_domain"
    }
  },
  "status": "ok",
  "time_exec": 0.13046002388,
  "time_server": 1556451492
}"#;

            let station_data: ::std::result::Result<StationData, _> = serde_json::from_str(&json);

            assert_that(&station_data).is_ok();
        }
    }
}
