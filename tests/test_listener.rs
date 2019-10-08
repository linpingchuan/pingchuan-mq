use std::io::prelude::*;
#[test]
fn test_connect() {
    let mut socket = std::net::TcpStream::connect("localhost:8800").unwrap();
    socket.write(b"hi").unwrap();
    let mut buf=String::from("");
    socket.read_to_string(&mut buf).unwrap();
    assert_eq!(buf,"hi, I am pingchuan :)")
}
