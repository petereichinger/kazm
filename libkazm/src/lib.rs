use std::{io, thread};
use std::io::{Error, Write};
use std::net::{SocketAddrV4, TcpListener};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use log::{debug, error, info};

use crate::request::header::Header;
use crate::request::pathmatcher::parse_path;
use crate::response::status_code::StatusCode;
use crate::response::response_writer::{write_response, write_empty_response};

mod request;
mod response;

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
        let run = Arc::new(AtomicBool::new(true));
        let run_clone = run.clone();
        if let Err(e) = ctrlc::set_handler(move || {
            info!("Closing web server");
            run_clone.store(false, Ordering::Relaxed);
        }) {
            error!("Could not add Ctrl+C handler: {}", e);
            return Err(Error::new(io::ErrorKind::Other, "Ctrl-C handler could not be added"));
        };

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

        if let Err(e) = listener.set_nonblocking(true) {
            error!("Could not set listener to non-blocking {}", e);
            return Err(e);
        }

        for stream in listener.incoming() {
            if !run.load(Ordering::Relaxed) {
                break;
            }

            match stream {
                Err(e) if e.kind() != io::ErrorKind::WouldBlock => { error!("Error while opening connection: {}", e); }
                Ok(mut stream) => {
                    debug!("Got connection from {}", match stream.peer_addr() {
                        Ok(addr) => { addr.to_string() }
                        Err(err) => { err.to_string() }
                    });
                    thread::spawn(move || {
                        match Header::get(&mut stream) {
                            Ok(_headers) => {
                                let path_result = parse_path(&_headers.path);

                                match path_result {
                                    Ok((path, params)) => {
                                        info!("{} {:?}", path, params);
                                        write_empty_response(&mut stream, StatusCode::OK).unwrap();
                                    }
                                    Err(e) => {
                                        error!("Error while parsing request. {}", e);
                                        write_empty_response(&mut stream, StatusCode::BadRequest).unwrap();
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Encountered error while parsing headers {}", e);
                                write_empty_response(&mut stream, StatusCode::BadRequest).unwrap();
                            }
                        }
                    });
                }
                _ => {}
            }
        }

        Ok(())
    }
}

