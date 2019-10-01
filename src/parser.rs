#[derive(Debug)]
pub struct PingchuanParser {}
#[derive(Debug, Clone)]
pub struct PingchuanEvent {
    request_content: String,
}
impl PingchuanParser {
    pub fn parse(&mut self, request_content: String) -> PingchuanEvent {
        let pingchuan_event = PingchuanEvent::of(request_content);
        pingchuan_event
    }

    pub fn of() -> PingchuanParser {
        PingchuanParser {}
    }
}

impl PingchuanEvent {
    pub fn of(request_content: String) -> PingchuanEvent {
        PingchuanEvent { request_content }
    }
}
