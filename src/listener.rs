use crate::parser;
use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct PingchuanListener {
    port: u16,
}

impl PingchuanListener {
    pub fn of(port: u16) -> PingchuanListener {
        PingchuanListener { port: port }
    }
    pub fn listen(&mut self) {
        let addrs = std::net::SocketAddr::from(([127, 0, 0, 1], self.port));
        let pingchuan_listener = std::net::TcpListener::bind(addrs).unwrap();

        for stream in pingchuan_listener.incoming() {
            match stream {
                Ok(stream) => {
                    let (tx, rx) = std::sync::mpsc::channel();
                    std::thread::spawn(move || {
                        match PingchuanListener::handle_connection(stream) {
                            Ok(pingchuan_event) => tx.send(pingchuan_event).unwrap(),
                            Err(_) => {}
                        }
                    });
                    match rx.recv() {
                        Ok(received) => {
                            println!("{:?}", received);
                        }
                        Err(_) => {}
                    }
                }
                Err(e) => {
                    println!("connecting error: {}", e);
                }
            }
        }
    }
    // 处理请求
    fn handle_connection(
        mut stream: std::net::TcpStream,
    ) -> Result<parser::PingchuanEvent, String> {
        const MAX_LENGTH: usize = 64;
        let mut buffer = [0; MAX_LENGTH];
        let mut len = stream.read(&mut buffer).unwrap();
        if len == 2 {
            let content = String::from_utf8_lossy(&buffer[..len]);
            let request_content = content.into_owned();
            if "hi" == request_content {
                return Ok(PingchuanListener::hi(stream, request_content));
            }
        }
        let mut buffer_vec: Vec<u8> = Vec::new();
        loop {
            buffer_vec.extend(buffer.to_vec());
            buffer = [0; MAX_LENGTH];
            len = stream.read(&mut buffer).unwrap();
            if len <= 0 || len < MAX_LENGTH {
                break;
            }
        }
        buffer_vec.extend(buffer.to_vec());
        let pingchuan_packet = Rc::new(RefCell::new(parser::PingchuanPacket::new()));
        match parser::PingchuanParser::deserialize_from_pingchuan_packet(
            pingchuan_packet,
            &mut buffer_vec,
        ) {
            Some(p) => {
                let packet = p.borrow();
                println!("{:?}", packet);
                let mut bytes = Vec::new();
                stream
                    .write(parser::PingchuanParser::serialize_to_pingchuan_packet(
                        p.clone(),
                        &mut bytes,
                    ))
                    .unwrap();
            }
            _ => println!("服务器出错"),
        }
        // stream.write(b"hi, I am pingchuan23333 :)").unwrap();
        // stream.flush().unwrap();
        // pingchuan_event
        Err(String::from("平川流无法解析"))
    }

    fn hi(mut stream: std::net::TcpStream, request_content: String) -> parser::PingchuanEvent {
        let mut pingchuan_parser = parser::PingchuanParser::of();
        let pingchuan_event = pingchuan_parser.parse(request_content);
        stream.write(b"hi, I am pingchuan :)").unwrap();
        stream.flush().unwrap();
        pingchuan_event
    }
}
