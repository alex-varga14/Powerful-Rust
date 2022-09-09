use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};
use rayon::prelude::*;
// Once you have discovered what servers are publicly available, you need to find out 
// what services are publicly available on those servers; PORT SCANNING!!

// Many different ways to scan ports; depends on what you desire, stealth, speed, reliability, and so on
// This program uses the simplest method, trying to open a TCP socket.
//          technique known as TCP connect.
// Good Analogy, Socket == Internet Pipe

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();
    
    // if socket_addresses.len() == 0 {
    //     return subdomain;
    // }
    if socket_addresses.is_empty() {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) //filter closed ports
        .collect();
    
    subdomain    
}

// Sequential scanning brutally slow, must implement parallel or it will take # of scanned ports * timeout seconds
// Use Multithreading
pub fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok(); 
    
    Port { port, is_open}
    // let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_address, timeout) {
    //     true
    // } else {
    //     false
    // };

    // Port {
    //     port: port,
    //     is_open,
    // }
} 