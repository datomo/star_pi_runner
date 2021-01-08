use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

struct Server {
    server: TcpListener
}

impl Server{
    pub(crate) fn new() -> Server {
        let server = TcpListener::bind("127:0.0.1:9001").unwrap();
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
            })
        }
        Server{server}
    }


}