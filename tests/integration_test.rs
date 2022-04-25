mod tests {
    use std::error::Error;

    use libkbat::client::{Client, Config};

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[test]
    fn test_get_metadata() -> Result<(), Box<dyn Error>> {
        init_logger();
        let config = Config::new();
        let mut client = Client::connect(config)?;
        let meta = client.refresh_metadata();
        log::debug!("metadata: {:#?}", meta);

        client.close();
        Ok(())
    }
}
