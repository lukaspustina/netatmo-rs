use netatmo_rs::{
    get_measure::{Parameters, Scale, Type},
    ClientCredentials, Netatmo, NetatmoClient, Scope,
};
use std::env;

fn main() {
    let client_id = env::var_os("NETATMO_CLIENT_ID")
        .expect("Environment variable 'NETATMO_CLIENT_ID' is not set.")
        .to_string_lossy()
        .to_string();
    let client_secret = env::var_os("NETATMO_CLIENT_SECRET")
        .expect("Environment variable 'NETATMO_CLIENT_SECRET' is not set.")
        .to_string_lossy()
        .to_string();
    let username = env::var_os("NETATMO_USERNAME")
        .expect("Environment variable 'NETATMO_USERNAME' is not set.")
        .to_string_lossy()
        .to_string();
    let password = env::var_os("NETATMO_PASSWORD")
        .expect("Environment variable 'NETATMO_PASSWORD' is not set.")
        .to_string_lossy()
        .to_string();
    let device_id = env::var_os("NETATMO_DEVICE_ID")
        .expect("Environment variable 'NETATMO_DEVICE_ID' is not set")
        .to_string_lossy()
        .to_string();

    let client_credentials = ClientCredentials {
        client_id: &client_id,
        client_secret: &client_secret,
    };
    let scopes = vec![Scope::ReadStation];
    let m_params = Parameters::new(&device_id, Scale::Max, &[Type::Humidity, Type::Temperature, Type::CO2]);

    let station_data = NetatmoClient::new(&client_credentials)
        .authenticate(&username, &password, &scopes)
        .expect("Failed to authenticate")
        .get_measure(&m_params)
        .expect("Failed to get measure");

    println!("{:#?}", station_data);
}
