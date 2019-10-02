fn main() {
    let mut pingchuan_server =
        pingchuan::broadcast_server::BroadcastServer::of(String::from("-p 8089"));
    pingchuan_server.start();
}
