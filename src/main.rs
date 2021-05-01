use std::io;
use std::io::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

use log::{error, info};
use simple_logger::SimpleLogger;

use libkazm::WebServer;

fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();


    let port = 8080;
    let address = Ipv4Addr::new(127, 0, 0, 1);
    let bind_address = SocketAddrV4::new(address, port);

    let server = Arc::new(WebServer::new(bind_address));

    let server_clone = server.clone();


    if let Err(e) = ctrlc::set_handler(move || {
        info!("Closing web server");
        server_clone.stop();
    }) {
        error!("Could not add Ctrl+C handler: {}", e);
        return Err(Error::new(io::ErrorKind::Other, "Ctrl-C handler could not be added"));
    };

    server.run()
}
