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

    pub fn serialize(bytes: &mut [u8]) -> &[u8] {
        // let  mut bytes:[u8;512]=[0;512];
        let magic_bytes = b"pingchuan";
        let mut capacity = 0;
        for index in 0..magic_bytes.len() {
            bytes[index] = magic_bytes[index];
        }
        capacity += magic_bytes.len();
        assert_eq!(bytes[..magic_bytes.len()], magic_bytes[..magic_bytes.len()]);
        let transaction_id: u64 = 13534044911;
        unsafe {
            let transaction_id_bytes = std::mem::transmute::<u64, [u8; 8]>(transaction_id);
            for index in 0..transaction_id_bytes.len() {
                bytes[capacity + index] = transaction_id_bytes[index];
            }
            capacity += transaction_id_bytes.len();
            assert_eq!(
                bytes[magic_bytes.len()..magic_bytes.len() + transaction_id_bytes.len()],
                transaction_id_bytes[..transaction_id_bytes.len()]
            );
        }
        bytes
    }
}

impl PingchuanEvent {
    pub fn of(request_content: String) -> PingchuanEvent {
        PingchuanEvent { request_content }
    }
}
