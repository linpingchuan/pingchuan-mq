use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct PingchuanParser {}
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct PingchuanEvent {
    request_content: String,
}
#[derive(Debug, Clone)]
pub struct PingchuanPacket {
    pub transaction_id: u64,
    pub topic_len: u64,
    pub content_len: u64,
    pub role: u64,

    pub order: u64,
    pub gzip: u64,
    pub offset: u64,

    pub topic: String,
    pub content: Rc<RefCell<Vec<u8>>>,
}

impl PingchuanPacket {
    pub fn new() -> PingchuanPacket {
        PingchuanPacket {
            transaction_id: 0,
            topic_len: 0,
            content_len: 0,
            role: 0,

            order: 0,
            gzip: 0,
            // crc: String::from(""),
            offset: 0,

            topic: String::from(""),
            content: Rc::new(RefCell::new(Vec::new())),
        }
    }
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

    fn add_u64_to_bytes(t: u64, bytes: &mut Vec<u8>) -> &mut Vec<u8> {
        unsafe {
            let tmps = std::mem::transmute::<u64, [u8; 8]>(t);
            for byte in tmps.iter() {
                bytes.push(*byte);
            }
        }
        bytes
    }

    fn from_bytes_to_64(bytes: Vec<u8>) -> u64 {
        unsafe {
            let mut tmps: [u8; 8] = [0; 8];
            for index in 0..bytes.len() {
                tmps[index] = bytes[index];
            }
            let tmps = std::mem::transmute::<[u8; 8], u64>(tmps);
            return tmps;
        }
    }

    pub fn deserialize_from_pingchuan_packet(
        pingchuan_packet: Rc<RefCell<PingchuanPacket>>,
        bytes: &mut Vec<u8>,
    ) -> Option<Rc<RefCell<PingchuanPacket>>> {
        let magic_bytes = b"pingchuan";
        let mut packet = pingchuan_packet.borrow_mut();
        // 检验魔数
        for index in 0..magic_bytes.len() {
            if bytes[index] != magic_bytes[index] {
                return None;
            }
        }
        let (_, tmp) = bytes.split_at(magic_bytes.len());

        let split_length = 8;

        let (transaction_id, tmp) = tmp.split_at(split_length);
        packet.transaction_id = PingchuanParser::from_bytes_to_64(transaction_id.to_vec());

        let (topic_len, tmp) = tmp.split_at(split_length);
        packet.topic_len = PingchuanParser::from_bytes_to_64(topic_len.to_vec());

        let (content_len, tmp) = tmp.split_at(split_length);
        packet.content_len = PingchuanParser::from_bytes_to_64(content_len.to_vec());

        let (role, tmp) = tmp.split_at(split_length);
        packet.role = PingchuanParser::from_bytes_to_64(role.to_vec());

        let (order, tmp) = tmp.split_at(split_length);
        packet.order = PingchuanParser::from_bytes_to_64(order.to_vec());

        let (gzip, tmp) = tmp.split_at(split_length);
        packet.gzip = PingchuanParser::from_bytes_to_64(gzip.to_vec());

        // let (crc, tmp) = tmp.split_at(split_length);
        // packet.crc = String::from_utf8_lossy(&crc).into_owned();

        let (offset, tmp) = tmp.split_at(split_length);
        packet.offset = PingchuanParser::from_bytes_to_64(offset.to_vec());

        let (topic, tmp) = tmp.split_at(packet.topic_len as usize);
        packet.topic = String::from_utf8_lossy(&topic).into_owned();

        let (content, _) = tmp.split_at(packet.content_len as usize);
        packet.content = Rc::new(RefCell::new(content.to_vec()));
        Some(pingchuan_packet.clone())
    }

    pub fn serialize_to_pingchuan_packet(
        p: Rc<RefCell<PingchuanPacket>>,
        bytes: &mut Vec<u8>,
    ) -> &mut Vec<u8> {
        let packet = p.borrow();
        let magic_bytes = b"pingchuan";
        for index in 0..magic_bytes.len() {
            bytes.push(magic_bytes[index]);
        }
        let mut topic_len: u64 = 0;
        for _ in packet.topic.bytes() {
            topic_len += 1;
        }
        let result = PingchuanParser::add_u64_to_bytes(packet.transaction_id, bytes);
        let result = PingchuanParser::add_u64_to_bytes(topic_len, result);
        // content
        let content_len = packet.content.borrow().len() as u64;
        let result = PingchuanParser::add_u64_to_bytes(content_len, result);
        let result = PingchuanParser::add_u64_to_bytes(packet.role, result);
        let result = PingchuanParser::add_u64_to_bytes(packet.order, result);
        let result = PingchuanParser::add_u64_to_bytes(packet.gzip, result);
        // crc
        // for byte in packet.crc.bytes() {
        //     result.push(byte);
        // }
        let result = PingchuanParser::add_u64_to_bytes(packet.offset, result);

        for byte in packet.topic.bytes() {
            result.push(byte);
        }
        // content
        let tmp = packet.content.borrow();
        for byte in tmp.iter() {
            result.push(*byte);
        }
        result
    }
}

impl PingchuanEvent {
    pub fn of(request_content: String) -> PingchuanEvent {
        PingchuanEvent { request_content }
    }
}
