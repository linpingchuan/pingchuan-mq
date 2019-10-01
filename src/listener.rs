use crate::parser;
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
            let stream = stream.unwrap();
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let pingchuan_event = PingchuanListener::handle_connection(stream);
                tx.send(pingchuan_event).unwrap();
            });
            let received = rx.recv().unwrap();
            println!("{:?}", received);
        }
    }
    // 处理请求
    fn handle_connection(mut stream: std::net::TcpStream) -> parser::PingchuanEvent {
        println!("Connection established!");
        use std::io::prelude::*;
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let content = String::from_utf8_lossy(&buffer[..]);
        let mut pingchuan_parser = parser::PingchuanParser::of();
        pingchuan_parser.parse(content.into_owned())
    }
}
