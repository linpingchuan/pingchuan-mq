use pingchuan::parser;
use std::io::prelude::*;
#[test]
fn test_connect() {
    let mut socket = std::net::TcpStream::connect("localhost:8800").unwrap();
    socket.write(b"hi").unwrap();
    let mut buf = String::from("");
    socket.read_to_string(&mut buf).unwrap();
    assert_eq!(buf, "hi, I am pingchuan :)")
}
#[test]
fn test_protocol() {
    let mut socket = std::net::TcpStream::connect("localhost:8800").unwrap();
    let mut packet = parser::PingchuanPacket {
        transaction_id: 1111,
        topic_len: 111,
        content_len: 111,
        role: 111,
        order: 111,
        gzip: 111,
        crc: String::from("111"),
        offset: 111,
        topic: String::from("111"),
        content: vec![12],
    };
    let mut bytes:Vec<u8>=Vec::new();
    let content=parser::PingchuanParser::serialize_to_pingchuan_packet(packet,&mut bytes);
    socket.write(content).unwrap();
}
