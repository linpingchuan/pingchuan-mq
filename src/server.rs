use crate::log;

pub struct Server {
    args: String,
    version: String,
    pingchuan_log: log::PingchuanLog,
}

impl Server {
    // 启动服务器
    pub fn start(&mut self) {
        print!("accept args: {}, ", self.args);
        println!("pingchuan-mq version: {} starting now...", self.version);
        // 从日志恢复处理过程
        self.pingchuan_log.read_wal_log();
        self.handle_connection();
        // 日志引擎开始处理日志
        self.pingchuan_log.accept();

    }
    // 创建服务器单实例
    pub fn of(args: String) -> Server {
        let pingchuan_server = Server {
            args: args,
            version: String::from("0.0.1"),
            pingchuan_log: log::PingchuanLog::of(),
        };
        pingchuan_server
    }
    // 处理请求
    pub fn handle_connection(&mut self){

    }
}
