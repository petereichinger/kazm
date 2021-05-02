use std::{io, thread};
use std::collections::HashMap;
use std::io::Error;
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

use log::{error, info};

use request::header::Header;
use request::pathmatcher::parse_path;
use response::response_writer::write_empty_response;
use response::status_code::StatusCode;

mod request;
pub mod response;

#[derive(Debug)]
pub enum CallbackError {
    AlreadyRegistered,
    NoCallbackForPath,
}

pub struct CallbackHandler {
    callbacks: HashMap<String, Box<dyn Fn() -> StatusCode + Sync + Send>>,
}

impl Default for CallbackHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CallbackHandler {
    pub fn new() -> CallbackHandler {
        CallbackHandler { callbacks: HashMap::new() }
    }

    pub fn register(&mut self, path: &str, func: Box<dyn Fn() -> StatusCode + Sync + Send>) -> Result<(), CallbackError> {
        if self.callbacks.contains_key(path) {
            return Err(CallbackError::AlreadyRegistered);
        }

        self.callbacks.insert(String::from(path), func);

        Ok(())
    }


    pub fn handle(&self, path: &str) -> Result<StatusCode, CallbackError> {
        match self.callbacks.get(path) {
            None => {
                error!("No callback found for path {}", path);
                Err(CallbackError::NoCallbackForPath)
            }
            Some(func) => Ok(func())
        }
    }
}


/// A simple web server that currently only verifies if a request has well formatted parameters
pub struct WebServer {
    address: SocketAddrV4,
    running: RwLock<bool>,
    callback_handler: Arc<RwLock<CallbackHandler>>,
}

impl WebServer {
    /// Create a new instance of the web server.
    pub fn new(address: SocketAddrV4) -> WebServer {
        WebServer {
            address,
            running: RwLock::new(true),
            callback_handler: Arc::new(RwLock::new(CallbackHandler::new())),
        }
    }

    /// Get the current address the web server is bound to
    pub fn address(&self) -> SocketAddrV4 {
        self.address
    }

    pub fn register_callback(&self, path: &str, func: Box<dyn Fn() -> StatusCode + Sync + Send>) -> Result<(), CallbackError> {
        info!("Registering callback for path {}", path);
        self.callback_handler.write().unwrap().register(path, func)
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

        let mut last_run_check = std::time::Instant::now();
        let check_interval = std::time::Duration::from_millis(250);
        for stream in listener.incoming() {
            let current_run_check = std::time::Instant::now();
            if (current_run_check - last_run_check) > check_interval {
                let running_reader = self.running.read().unwrap();

                if !*running_reader {
                    break;
                }

                last_run_check = current_run_check
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
        let cb_handler = self.callback_handler.clone();
        thread::spawn(
            move || {
                match Header::get(&mut stream)
                {
                    Ok(_headers) => {
                        let path_result = parse_path(&_headers.path);

                        match path_result {
                            Ok((path, params)) => {
                                info!("{} {:?}", path, params);
                                let code = cb_handler.read().unwrap().handle(&path).unwrap_or(StatusCode::NotFound);
                                write_empty_response(&mut stream, code).unwrap();
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

