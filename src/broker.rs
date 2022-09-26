use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::error::{Error, Result};
use crate::model::ClusterMetadata;
use crate::request::{GetMetadataRequest, Request};

pub struct Broker {
    pub addr: String,
    connect_timeout: Duration,
    connection: Option<Box<TcpStream>>,
}

impl Broker {
    pub fn connect(addr: &str, timeout: Duration) -> Result<Broker> {
        log::debug!("connecting to broker: {}", addr);
        let mut broker = Broker { addr: String::from(addr), connect_timeout: timeout, connection: None };
        broker.reconnect()?;
        Ok(broker)
    }

    fn reconnect(&mut self) -> Result<()> {
        let mut last_error = None;
        let addrs_iter = self.addr.to_socket_addrs()?;
        for socket_addr in addrs_iter {
            match TcpStream::connect_timeout(&socket_addr, self.connect_timeout) {
                Ok(stream) => {
                    log::debug!("connected to {}", self.addr);
                    self.connection = Some(Box::new(stream));
                    return Ok(());
                }
                Err(e) => {
                    log::debug!("error connecting to {}: {}", self.addr, e);
                    last_error = Some(e.into());
                }
            }
        }

        Err(last_error.unwrap())
    }

    pub fn get_metadata(&self) -> Result<ClusterMetadata> {
        let request = GetMetadataRequest {};
        return self.send_and_receive(request);
    }

    pub fn close(&mut self) {
        // let stream = self.stream;
        match &self.connection.as_deref() {
            None => None::<()>,
            Some(mut s) => match s.flush() {
                Ok(()) => None,
                Err(e) => {
                    log::warn!("error closing broker {}: {}", self.addr, e);
                    None
                }
            },
        };
        self.connection = None;
    }

    fn send_and_receive(&self, req: GetMetadataRequest) -> Result<ClusterMetadata> {
        self.send(req)?;
        todo!("receive")
    }

    fn connection(&self) -> Result<&TcpStream> {
        let conn: &TcpStream;
        match &self.connection {
            Some(c) => conn = &c,
            None => return Err(Error::NotConnected),
        }
        Ok(conn)
    }

    fn send(&self, req: GetMetadataRequest) -> Result<()> {
        let body = req.into_bytes()?;
        let body = body.as_slice();
        self.connection()?.write(body)?;
        Ok(())
    }
}
