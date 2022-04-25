use std::time::Duration;

use crate::broker::Broker;
use crate::error::{Error, Result};
use crate::model::ClusterMetadata;

pub struct Config {
    pub brokers: Vec<String>,
    pub connect_timeout: Duration,
}

impl Config {
    pub fn new() -> Config {
        Config {
            brokers: vec![String::from("localhost:9092")],
            connect_timeout: Duration::from_secs(10),
        }
    }
}

pub struct Client {
    config: Config,
    brokers: Vec<Broker>,
    cluster_metadata: Option<Box<ClusterMetadata>>,
}

impl Client {
    pub fn connect(config: Config) -> Result<Client> {
        let mut client = Client {
            config,
            brokers: vec![],
            cluster_metadata: None,
        };
        client.reconnect()?;
        Ok(client)
    }

    pub fn refresh_metadata(&mut self) -> Result<()> {
        if self.brokers.is_empty() {
            return Err(Error::NoBrokers);
        }

        let mut last_error = None;
        for broker in &self.brokers {
            match broker.get_metadata() {
                Ok(cm) => {
                    self.cluster_metadata = Some(Box::new(cm));
                    return Ok(());
                }
                Err(e) => last_error = Some(e)
            }
        }

        return Err(last_error.unwrap());
    }

    fn reconnect(&mut self) -> Result<()> {
        let timeout = self.config.connect_timeout;
        let mut brokers = vec![];
        for addr in self.config.brokers.as_slice() {
            let broker = Broker::connect(addr, timeout)?;
            brokers.push(broker)
        }
        self.brokers = brokers;

        Ok(())
    }

    pub fn close(&mut self) {
        for broker in self.brokers.iter_mut() {
            broker.close()
        }
        self.brokers = vec![]
    }
}
