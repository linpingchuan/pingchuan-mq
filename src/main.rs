fn main() {
    let mut pingchuan_server = pingchuan::server::Server::of(String::from("-p 8089"));
    pingchuan_server.start();
}
