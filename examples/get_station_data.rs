use netatmo_rs::{Scope, Settings, get_token, get_station_data};
use std::env;

fn main() {
    let cliend_id = env::var_os("NETATMO_CLIENT_ID").expect("Environment variable 'NETATMO_CLIENT_ID' is not set.");
    let client_secret = env::var_os("NETATMO_CLIENT_SECRET").expect("Environment variable 'NETATMO_CLIENT_SECRET' is not set.");
    let username = env::var_os("NETATMO_USERNAME").expect("Environment variable 'NETATMO_USERNAME' is not set.");
    let password = env::var_os("NETATMO_PASSWORD").expect("Environment variable 'NETATMO_PASSWORD' is not set.");
    let device_id = env::var_os("NETATMO_DEVICE_ID").expect("Environment variable 'NETATMO_DEVICE_ID' is not set.");

    let settings = Settings {
        cliend_id: &cliend_id.to_string_lossy(),
        client_secret: &client_secret.to_string_lossy(),
        username: &username.to_string_lossy(),
        password: &password.to_string_lossy(),
    };
    let scopes = vec![Scope::ReadStation];

    let token = get_token(&settings, &scopes);
    let station_data = get_station_data(&token, &device_id.to_string_lossy());

    println!("{:#?}", station_data);
}

