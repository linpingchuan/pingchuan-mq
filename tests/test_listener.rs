use std::io::prelude::*;
#[test]
fn test_connect() {
    let mut socket = std::net::TcpStream::connect("localhost:8800").unwrap();
    socket.write(b"Hello pingchuan");
}
