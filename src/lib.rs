pub mod client;
pub mod errors;

pub use client::authenticate::{self, Scope};
pub use client::get_home_status;
pub use client::get_homes_data;
pub use client::get_measure;
pub use client::get_station_data;
pub use client::set_room_thermpoint;
pub use client::{ClientCredentials, Netatmo, NetatmoClient};
