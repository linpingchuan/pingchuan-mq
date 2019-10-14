use pingchuan::parser;
use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;

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
    let packet = parser::PingchuanPacket {
        transaction_id: 1111,
        topic_len: 111,
        content_len: 111,
        role: 111,
        order: 111,
        gzip: 111,
        crc: String::from("111"),
        offset: 111,
        topic: String::from("111"),
        content: Rc::new(RefCell::new(vec![12])),
    };
    let mut bytes: Vec<u8> = Vec::new();
    let content = parser::PingchuanParser::serialize_to_pingchuan_packet(
        Rc::new(RefCell::new(packet)),
        &mut bytes,
    );
    socket.write(content).unwrap();
    let mut bytes = [0; 1024];
    socket.read(&mut bytes).unwrap();
    let mut bytes = bytes.to_vec();
    match parser::PingchuanParser::deserialize_from_pingchuan_packet(
        Rc::new(RefCell::new(parser::PingchuanPacket::new())),
        &mut bytes,
    ) {
        Some(packet) => {
            assert_eq!(packet.borrow().transaction_id, 1111);
        }
        _ => assert!(false),
    }
}
