#[derive(Debug)]
pub struct PingchuanParser {}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PingchuanEvent {
    request_content: String,
}
#[derive(Debug)]
pub struct PingchuanPacket{
    pub transaction_id:u64,
    pub topic_len:u64,
    pub content_len:u64, 
    pub role:u64 ,
    pub order:u64, 
    pub gzip:u64,
    pub crc:String,
    pub offset:u64,
    pub topic:String,
    pub content:Vec<u8>, 
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

    fn from_bytes_to_64(bytes:Vec<u8>)->u64{
        unsafe{
            let mut tmps:[u8;8]=[0;8];
            for index in 0..bytes.len(){
                tmps[index]=bytes[index];
            }
            let tmps = std::mem::transmute::<[u8;8],u64>(tmps);
            return tmps;
        }
        0
    }

    pub fn deserialize_from_pingchuan_packet(packet:mut PingchuanPacket,bytes:&mut Vec<u8>)->Option<PingchuanPacket>{
        let magic_bytes = b"pingchuan";
        // 检验魔数
        for index in 0..magic_bytes.len() {
            if bytes[index]!=magic_bytes[index]{
                return None
            }    
        }
        let (_,tmp)=bytes.split_at(magic_bytes.len());

        let split_length=8;

        let (transaction_id,tmp)=tmp.split_at(split_length);
        packet.transaction_id=PingchuanParser::from_bytes_to_64(transaction_id.to_vec());
        
        let (topic_len,tmp)=tmp.split_at(split_length);
        packet.topic_len=PingchuanParser::from_bytes_to_64(topic_len.to_vec());

        let (content_len,tmp)=tmp.split_at(split_length);
        packet.content_len=PingchuanParser::from_bytes_to_64(content_len.to_vec());

        let (role,tmp)=tmp.split_at(split_length);
        packet.role=PingchuanParser::from_bytes_to_64(role.to_vec());

        let (order,tmp)=tmp.split_at(split_length);
        packet.order=PingchuanParser::from_bytes_to_64(order.to_vec());

        let (gzip,tmp)=tmp.split_at(split_length);
        packet.gzip=PingchuanParser::from_bytes_to_64(gzip.to_vec());

        let (crc,tmp)=tmp.split_at(split_length);
        packet.gzip=PingchuanParser::from_bytes_to_64(crc.to_vec());

        let (offset,tmp)=tmp.split_at(split_length);
        packet.offset=PingchuanParser::from_bytes_to_64(offset.to_vec());
        
        // let result=bytes.truncate(len: usize)
        None
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
