use std::io::Error;
use std::net::{SocketAddrV4, TcpListener};
use std::thread;

use log::{debug, error, info};

mod request;

/// A simple web server that currently does not respond with any message whatsoever.
pub struct WebServer {
    address: SocketAddrV4
}

impl WebServer {
    /// Create a new instance of the web server.
    pub fn new(address: SocketAddrV4) -> WebServer {
        WebServer { address }
    }

    /// Get the current address the web server is bound to
    pub fn address(&self) -> SocketAddrV4 {
        self.address
    }

    /// Run the web server.
    /// This function returns with an error, if binding to the specified socket is not possible see [Self::address()]
    pub fn run(&self) -> Result<(), Error> {
        info!("Trying to bind to {}", self.address);

        let listener = match TcpListener::bind(self.address) {
            Ok(listener) => {
                info!("Successfully bound to {}", self.address);
                listener
            }
            Err(e) => {
                error!("Could not bind to {}: {}", self.address, e);
                return Err(e);
            }
        };

        for stream in listener.incoming() {
            match stream {
                Err(e) => { error!("Error while opening connection: {}", e); }
                Ok(stream) => {
                    debug!("Got connection from {}", match stream.peer_addr() {
                        Ok(addr) => { addr.to_string() }
                        Err(err) => { err.to_string() }
                    });
                    thread::spawn(move || { request::handler::handle(stream) });
                }
            }
        }

        Ok(())
    }
}

