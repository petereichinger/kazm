use std::io::Error;
use std::net::{Ipv4Addr, SocketAddrV4};

use simple_logger::SimpleLogger;

use libkazm::WebServer;

fn main() -> Result<(), Error> {
    SimpleLogger::new().init().unwrap();

    let port = 8080;
    let address = Ipv4Addr::new(127, 0, 0, 1);
    let bind_address = SocketAddrV4::new(address, port);

    let server = WebServer::new(bind_address);

    server.run()
}
