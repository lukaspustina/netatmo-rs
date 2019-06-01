pub mod client;
pub mod errors;

pub use client::{ClientCredentials, Netatmo, NetatmoClient};
pub use client::authenticate::{self, Scope};
pub use client::get_station_data;
pub use client::get_measure;