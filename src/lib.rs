pub mod client;
pub mod errors;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Measure {}

pub fn get_measure(token: &client::Token, device_id: &str) -> Measure {
    //cf. https://dev.netatmo.com/resources/technical/reference/common/getmeasure
    unimplemented!("NYI")
}
