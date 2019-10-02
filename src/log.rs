pub struct PingchuanLog {}

impl PingchuanLog {
    pub fn read_wal_log(&mut self) {
        println!("try recovery from log, wait a moment :)")
    }

    pub fn accept(&mut self) {}

    pub fn of() -> PingchuanLog {
        let pingchaun_log = PingchuanLog {};
        pingchaun_log
    }
}
