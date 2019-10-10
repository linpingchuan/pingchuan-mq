#[derive(Debug)]
pub struct PingchuanParser {}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PingchuanEvent {
    request_content: String,
}
#[derive(Debug)]
// #[repr(transparent)]
pub struct PingchuanPacket{
    transaction_id:u64,
    topic_len:u64,
    content_len:u64, 
    role:u64 ,
    order:u64, 
    gzip:u64,
    offset:u64,
    topic:String,
    content:Vec<u8>, 
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

    fn add_u64_to_bytes(t:u64,bytes:&mut [u8],offset:usize)->(&[u8],usize){
        let mut capacity=offset;
        unsafe {
            
            let transaction_id_bytes = std::mem::transmute::<u64, [u8; 8]>(t);
            for index in 0..transaction_id_bytes.len() {
                bytes[capacity + index] = transaction_id_bytes[index];
            }
            capacity += transaction_id_bytes.len();
            assert_eq!(
                bytes[offset..offset + transaction_id_bytes.len()],
                transaction_id_bytes[..transaction_id_bytes.len()]
            );
        }
        (bytes,capacity)
    }

    pub fn serialize_to_pingchuan_packet(bytes: &mut [u8]) -> (&[u8],usize) {
        // let  mut bytes:[u8;512]=[0;512];
        let magic_bytes = b"pingchuan";
        let mut capacity:usize = 0;
        for index in 0..magic_bytes.len() {
            bytes[index] = magic_bytes[index];
        }
        capacity += magic_bytes.len();
        assert_eq!(bytes[..magic_bytes.len()], magic_bytes[..magic_bytes.len()]);
        let transaction_id: u64 = 13534044911;
        let result=PingchuanParser::add_u64_to_bytes(transaction_id, bytes, capacity);
        result
    }
}

impl PingchuanEvent {
    pub fn of(request_content: String) -> PingchuanEvent {
        PingchuanEvent { request_content }
    }
}
