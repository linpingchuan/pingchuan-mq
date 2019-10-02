use crate::listener;
use crate::log;

pub struct BroadcastServer {
    args: String,
    version: String,
    pingchuan_log: log::PingchuanLog,
    pingchuan_listener: listener::PingchuanListener,
}

impl BroadcastServer {
    // 启动服务器
    pub fn start(&mut self) {
        print!("accept args: {}, ", self.args);
        println!("pingchuan-mq version: {} starting now...", self.version);
        // 从日志恢复处理过程
        self.pingchuan_log.read_wal_log();
        // 日志引擎开始处理日志
        self.pingchuan_log.accept();
        // 监听服务
        self.pingchuan_listener.listen();
    }
    // 创建服务器单实例
    pub fn of(args: String) -> BroadcastServer {
        let pingchuan_listener = listener::PingchuanListener::of(8800);
        let pingchuan_server = BroadcastServer {
            args: args,
            version: String::from("0.0.1"),
            pingchuan_log: log::PingchuanLog::of(),
            pingchuan_listener: pingchuan_listener,
        };
        pingchuan_server
    }
    
}
