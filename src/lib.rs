pub mod client;
pub mod errors;

pub use client::{
    authenticate::{self, Scope},
    get_home_status,
    get_homes_data,
    get_measure,
    get_station_data,
    set_room_thermpoint,
    ClientCredentials,
    Netatmo,
    NetatmoClient,
};
