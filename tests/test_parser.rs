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