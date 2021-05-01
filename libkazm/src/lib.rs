use std::{io, thread};
// use std::collections::HashMap;
use std::io::Error;
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::RwLock;

use log::{error, info};

use crate::request::header::Header;
use crate::request::pathmatcher::parse_path;
use crate::response::response_writer::write_empty_response;
use crate::response::status_code::StatusCode;

mod request;
mod response;

// struct CallbackHandler<CB> where CB: FnMut() {
//     callbacks: HashMap<String, CB>,
// }


/// A simple web server that currently does not respond with any message whatsoever.
pub struct WebServer {
    address: SocketAddrV4,
    running: RwLock<bool>,
}

impl WebServer {
    /// Create a new instance of the web server.
    pub fn new(address: SocketAddrV4) -> WebServer {
        WebServer { address, running: RwLock::new(true) }
    }

    /// Get the current address the web server is bound to
    pub fn address(&self) -> SocketAddrV4 {
        self.address
    }

    /// Stop the web server.
    /// Currently running requests will continue to run.
    /// Any further requests are ignored.
    pub fn stop(&self) {
        let mut n = self.running.write().unwrap();
        *n = false;
    }

    /// Run the web server.
    /// This function returns with an error, if binding to the specified socket is not possible see [Self::address()]
    /// The server runs in an infinite loop. To stop the server call [Self::stop()]
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

        if let Err(e) = listener.set_nonblocking(true) {
            error!("Could not set listener to non-blocking {}", e);
            return Err(e);
        }

        for stream in listener.incoming() {
            let running_reader = self.running.read().unwrap();

            if !*running_reader {
                break;
            }

            match stream {
                Err(e) if e.kind() != io::ErrorKind::WouldBlock => { error!("Error while opening connection: {}", e); }
                Ok(stream) => {
                    if let Ok(addr) = stream.peer_addr()
                    {
                        info!("Got connection from {}", addr);
                        self.handle_connection(stream);
                    } else {
                        error!("Error while initializing connection");
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        thread::spawn(move || {
            match Header::get(&mut stream)
            {
                Ok(_headers) => {
                    let path_result = parse_path(&_headers.path);

                    match path_result {
                        Ok((path, params)) => {
                            info!("{} {:?}", path, params);
                            write_empty_response(&mut stream, StatusCode::Ok).unwrap();
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
}

