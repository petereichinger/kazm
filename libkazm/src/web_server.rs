use std::{io, thread};
use std::io::{BufRead, BufReader, Error};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

use log::{error, info};

use crate::request::uri_parser::UriValues;
use crate::callback_handler::{CallbackHandler, CallbackError};
use crate::response::status_code::StatusCode;
use crate::request::header::Header;
use crate::request;
use crate::response::response_writer::write_empty_response;

/// A simple web server that supports setting static callback methods depending on the path.
///
/// If a path cannot be found the server returns 404 Not Found. Otherwise 200 OK.
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

    /// Register a callback function for a specified path
    ///
    /// # Parameters
    ///
    /// - `path`: Path to use
    /// - `func` Function to use as callback
    ///
    /// # Returns
    ///
    /// [`Ok`] if successful, or [`Err<CallbackError>`] in case something went wrong
    pub fn register_callback(&self, path: &str, func: Box<dyn Fn() -> StatusCode + Sync + Send>) -> Result<(), CallbackError> {
        info!("Registering callback for path {}", path);
        self.callback_handler.write().unwrap().register(path, func)
    }

    /// Unregister a previsously registered callback at the specified path
    ///
    /// # Parameters
    ///
    /// - `path` The path for the callback
    ///
    /// # Returns
    ///
    /// [`Ok`] if successful, or [`Err<CallbackError>`] in case something went wrong
    pub fn unregister_callback(&self, path: &str) -> Result<(), CallbackError> {
        info!("Unregistering callback for path {}", path);
        self.callback_handler.write().unwrap().unregister(path)
    }

    /// Stop the web server.
    ///
    /// Currently running requests will continue to run.
    /// Any further requests are ignored.
    pub fn stop(&self) {
        let mut n = self.running.write().unwrap();
        *n = false;
    }

    /// Run the web server.
    ///
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
                let stream_reader = BufReader::new(&stream);
                let mut lines = stream_reader.lines();
                match Header::parse(&mut lines)
                {
                    Ok(_headers) => {
                        let path_result = request::uri_parser::UriValues::from(&_headers.uri);

                        match path_result {
                            Ok(UriValues { path, parameters }) => {
                                info!("{} {:?}", path, parameters);
                                let code = cb_handler.read().unwrap().handle(&path).unwrap_or(StatusCode::NotFound);
                                write_empty_response(&mut stream, code).unwrap();
                            }
                            Err(e) => {
                                error!("Error while parsing request. {:?}", e);
                                write_empty_response(&mut stream, StatusCode::BadRequest).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        error!("Encountered error while parsing headers {}", e.to_string());
                        write_empty_response(&mut stream, StatusCode::BadRequest).unwrap();
                    }
                }
            });
    }
}
