use std::error::Error;

use libkbat::client::{Client, Config};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client_config = Config::new();
    let mut client = Client::connect(client_config)?;
    let metadata = client.refresh_metadata()?;
    log::info!("retrieved metadata!");
    log::info!("{:#?}", metadata);
    client.close();
    Ok(())
}
