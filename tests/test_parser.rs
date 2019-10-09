#[test]
fn test_serialize_event(){
    struct Mystruct{
        id:u8,
        data:[u8;1024]
    }
    let my_struct=Mystruct{id:0,data:[1;1024]};
    let bytes:&[u8]=unsafe{pingchuan::parser::PingchuanParser::any_as_u8_slice(&my_struct)};
    println!("{:?}",bytes)
}

#[test]
fn test_my_serializer(){
    let mut bytes:[u8;512]=[0;512];
    let magic_bytes=b"pingchuan";
    let mut capacity=0;
    for index in 0..magic_bytes.len(){
        bytes[index]=magic_bytes[index];
        
    }
    capacity+=magic_bytes.len();
    assert_eq!(bytes[..magic_bytes.len()],magic_bytes[..magic_bytes.len()]);
    let transaction_id:u32=43333;
    unsafe{
        let transaction_id_bytes=std::mem::transmute::<u32,[u8;4]>(transaction_id);
        for index in 0..transaction_id_bytes.len() {
            bytes[capacity+index]=transaction_id_bytes[index];
        }
        capacity+=transaction_id_bytes.len();
        assert_eq!(bytes[magic_bytes.len()..magic_bytes.len()+transaction_id_bytes.len()],transaction_id_bytes[..transaction_id_bytes.len()]);
    }
    
    // bytes[capacity+1]=transaction_id;
}