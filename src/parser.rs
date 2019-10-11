#[derive(Debug)]
pub struct PingchuanParser {}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PingchuanEvent {
    request_content: String,
}
#[derive(Debug)]
pub struct PingchuanPacket{
    transaction_id:u64,
    topic_len:u64,
    content_len:u64, 
    role:u64 ,
    order:u64, 
    gzip:u64,
    crc:String,
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

    fn add_u64_to_bytes(t:u64,bytes:&mut Vec<u8>)->&mut Vec<u8>{
        unsafe {
            let tmps = std::mem::transmute::<u64, [u8; 8]>(t);
            for byte in tmps.iter() {
                bytes.push(*byte);
            }
            
        }
        bytes
    }

    pub fn serialize_to_pingchuan_packet(packet:PingchuanPacket,bytes: &mut Vec<u8>) -> &mut Vec<u8> {
        let magic_bytes = b"pingchuan";
        for index in 0..magic_bytes.len() {
            bytes.push(magic_bytes[index]);
        }
        
        let result=PingchuanParser::add_u64_to_bytes(packet.transaction_id, bytes);
        let result=PingchuanParser::add_u64_to_bytes(packet.topic_len, result);
        let result=PingchuanParser::add_u64_to_bytes(packet.content_len, result);
        let result=PingchuanParser::add_u64_to_bytes(packet.role, result);
        let result=PingchuanParser::add_u64_to_bytes(packet.order, result);
        let result=PingchuanParser::add_u64_to_bytes(packet.gzip, result);
        // crc
        for byte in packet.crc.bytes(){
            result.push(byte);
        }
        let result=PingchuanParser::add_u64_to_bytes(packet.offset, result);
        let result=PingchuanParser::add_u64_to_bytes(packet.topic_len, result);
        for byte in packet.topic.bytes(){
            result.push(byte);
        }
        // content
        for byte in packet.content{
            result.push(byte);
        }
        result
    }
}

impl PingchuanEvent {
    pub fn of(request_content: String) -> PingchuanEvent {
        PingchuanEvent { request_content }
    }
}
