use crate::client::AuthenticatedClient;
use crate::errors::Result;

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

pub struct Parameters<'a> {
    device_id: &'a str,
    module_id: &'a str,
    scale: Scale,
    types: &'a [Type],
    date_begin: Option<usize>,
    date_end: Option<usize>,
    limit: Option<bool>,
    real_time: Option<bool>,
}

impl<'a> Parameters<'a> {
    pub fn new(device_id: &'a str, scale: Scale, types: &'a [Type]) -> Self {
        Parameters {
            device_id,
            module_id: device_id,
            scale,
            types,
            date_begin: None,
            date_end: None,
            limit: None,
            real_time: None,
        }
    }

    pub fn with_module_id(
        device_id: &'a str,
        module_id: &'a str,
        scale: Scale,
        types: &'a [Type],
    ) -> Self {
        Parameters {
            device_id,
            module_id,
            scale,
            types,
            date_begin: None,
            date_end: None,
            limit: None,
            real_time: None,
        }
    }

    pub fn date_begin(self, date_begin: usize) -> Self {
        Parameters {
            date_begin: Some(date_begin),
            ..self
        }
    }

    pub fn date_end(self, date_end: usize) -> Self {
        Parameters {
            date_end: Some(date_end),
            ..self
        }
    }

    pub fn limit(self, limit: bool) -> Self {
        Parameters {
            limit: Some(limit),
            ..self
        }
    }

    pub fn real_time(self, real_time: bool) -> Self {
        Parameters {
            real_time: Some(real_time),
            ..self
        }
    }
}

pub enum Scale {
    Max,
    Min30,
    Hour1,
    Hours3,
    Day1,
    Week1,
    Month1,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Scale::Max => "max",
            Scale::Min30 => "30min",
            Scale::Hour1 => "1hour",
            Scale::Hours3 => "3hours",
            Scale::Day1 => "1day",
            Scale::Week1 => "1week",
            Scale::Month1 => "1month",
        };
        write!(f, "{}", s)
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
        let types = p
            .types
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .as_slice()
            .join(",");
        let mut m = HashMap::default();
        m.insert("device_id", p.device_id.to_string());
        m.insert("module_id", p.module_id.to_string());
        m.insert("scale", p.scale.to_string());
        m.insert("type", types);
        if let Some(date_begin) = p.date_begin {
            m.insert("date_begin", date_begin.to_string());
        }
        if let Some(date_end) = p.date_end {
            m.insert("date_end", date_end.to_string());
        }
        if let Some(limit) = p.limit {
            m.insert("limit", limit.to_string());
        }
        m.insert("optimize", "false".to_string());
        if let Some(real_time) = p.real_time {
            m.insert("real_time", real_time.to_string());
        }

        m
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measure {
    status: String,
    time_exec: f64,
    #[serde(rename = "body", deserialize_with = "de_body_values")]
    values: HashMap<usize, Vec<Option<f64>>>,
}

//cf. https://dev.netatmo.com/resources/technical/reference/common/getmeasure
pub fn get_measure(client: &AuthenticatedClient, parameters: &Parameters) -> Result<Measure> {
    let params: HashMap<&str, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (*k, v.as_ref())).collect();

    client.call("https://api.netatmo.com/api/getmeasure", &mut params)
}

fn de_body_values<'de, D>(
    deserializer: D,
) -> ::std::result::Result<HashMap<usize, Vec<Option<f64>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map = HashMap::<String, Vec<Option<f64>>>::deserialize(deserializer)?;
    let mut tuples = Vec::new();
    for (k, v) in map {
        let key = usize::from_str(&k).map_err(serde::de::Error::custom)?;
        tuples.push((key, v));
    }
    let res = tuples.into_iter().collect();

    Ok(res)
}
