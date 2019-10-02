use std::io::prelude::*;
#[test]
fn test_connect() {
    let mut socket = std::net::TcpStream::connect("localhost:8800").unwrap();
    socket.write(b"Hello pingchuan");
    let mut buf=String::from("");
    socket.read_to_string(&mut buf);
    assert_eq!(buf,"hi")
}
