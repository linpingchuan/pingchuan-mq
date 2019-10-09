#[derive(Debug)]
pub struct PingchuanParser {}
#[derive(Debug, Clone)]
#[repr(transparent)]
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

    pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

impl PingchuanEvent {
    pub fn of(request_content: String) -> PingchuanEvent {
        PingchuanEvent { request_content }
    }
}
