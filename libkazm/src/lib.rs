pub mod method;
pub mod request_data;
pub mod request_handler;

use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use log::{info, error, debug};
use std::collections::HashMap;
use std::io::{Error, BufRead};
use crate::request_handler::RequestHandler;


pub struct WebServer {
    address: SocketAddrV4
}

impl WebServer {
    pub fn new(address: SocketAddrV4) -> WebServer {
        WebServer { address }
    }

    pub fn address(&self) -> SocketAddrV4 {
        self.address
    }

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
                    thread::spawn(move || { RequestHandler::new(stream).handle() });
                }
            }
        }

        Ok(())
    }
}

