use crate::{client::AuthenticatedClient, errors::Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct StationData {
    pub body: Body,
    pub status: String,
    pub time_exec: f64,
    pub time_server: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub devices: Vec<Device>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id")]
    pub id: String,
    pub co2_calibrating: bool,
    pub date_setup: u64,
    pub firmware: u64,
    pub last_setup: u64,
    pub last_status_store: u64,
    pub last_upgrade: Option<u64>,
    pub module_name: String,
    pub reachable: bool,
    pub station_name: String,
    #[serde(rename = "type")]
    pub type_info: String,
    pub wifi_status: f64,
    pub dashboard_data: DashboardData,
    pub data_type: Vec<String>,
    pub modules: Vec<Module>,
    pub place: Place,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardData {
    #[serde(rename = "AbsolutePressure")]
    pub absolute_pressure: Option<f64>,
    #[serde(rename = "CO2")]
    pub co2: Option<u64>,
    #[serde(rename = "Humidity")]
    pub humidity: Option<u64>,
    #[serde(rename = "Noise")]
    pub noise: Option<u64>,
    #[serde(rename = "Pressure")]
    pub pressure: Option<f64>,
    #[serde(rename = "Temperature")]
    pub temperature: Option<f64>,
    pub date_max_temp: Option<u64>,
    pub date_min_temp: Option<u64>,
    pub max_temp: Option<f64>,
    pub min_temp: Option<f64>,
    pub pressure_trend: Option<String>,
    pub temp_trend: Option<String>,
    pub time_utc: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    #[serde(rename = "_id")]
    pub id: String,
    pub battery_percent: u64,
    pub battery_vp: u64,
    pub dashboard_data: DashboardData,
    pub data_type: Vec<String>,
    pub firmware: u64,
    pub last_message: u64,
    pub last_seen: u64,
    pub last_setup: u64,
    pub module_name: String,
    pub reachable: bool,
    pub rf_status: u64,
    #[serde(rename = "type")]
    pub type_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    pub altitude: u64,
    pub city: String,
    pub country: String,
    pub location: Vec<f64>,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub administrative: Administrative,
    pub mail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Administrative {
    pub feel_like_algo: u64,
    pub lang: String,
    pub pressureunit: u64,
    pub reg_locale: String,
    pub unit: u64,
    pub windunit: u64,
}

pub(crate) fn get_station_data(client: &AuthenticatedClient, device_id: &str) -> Result<StationData> {
    let mut params: HashMap<&str, &str> = HashMap::default();
    params.insert("device_id", device_id);

    client.call("https://api.netatmo.com/api/getstationsdata", &mut params)
}

#[cfg(test)]
mod test {
    use super::*;
    use spectral::prelude::*;

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
