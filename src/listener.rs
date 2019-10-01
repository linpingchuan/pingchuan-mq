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
            std::thread::spawn(|| {
                PingchuanListener::handle_connection(stream);
            });
            
        }
    }
    // 处理请求
    fn handle_connection(mut stream: std::net::TcpStream) {
        println!("Connection established!");
        use std::io::prelude::*;
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
    }
}
