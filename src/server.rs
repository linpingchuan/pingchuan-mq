use crate::log;

pub struct Server {
    args: String,
    version: String,
    pingchuan_log: log::PingchuanLog,
    port: u16,
}

impl Server {
    // 启动服务器
    pub fn start(&mut self) {
        print!("accept args: {}, ", self.args);
        println!("pingchuan-mq version: {} starting now...", self.version);
        // 从日志恢复处理过程
        self.pingchuan_log.read_wal_log();
        // 日志引擎开始处理日志
        self.pingchuan_log.accept();
        // 监听服务
        self.listen();
    }
    // 创建服务器单实例
    pub fn of(args: String) -> Server {
        let pingchuan_server = Server {
            args: args,
            version: String::from("0.0.1"),
            pingchuan_log: log::PingchuanLog::of(),
            port: 8800,
        };
        pingchuan_server
    }
    fn listen(&mut self) {
        let addrs = std::net::SocketAddr::from(([127, 0, 0, 1], self.port));
        let pingchuan_listener = std::net::TcpListener::bind(addrs).unwrap();

        for stream in pingchuan_listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection();
            println!("Connection established!");
        }
    }
    // 处理请求
    fn handle_connection(&mut self) {}
}
