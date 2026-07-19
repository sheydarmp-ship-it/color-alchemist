use std::net::TcpListener;

pub struct Server {
    listener: TcpListener,
}
impl Server {
    pub fn new(addr: &str) -> Self {
        let listener =
            TcpListener::bind(addr).unwrap();

        Self { listener }
    }
    pub fn run(&self) {
    println!("Server started");

    for stream in self.listener.incoming() {
         match stream {

        Ok(stream) => {

            println!("Player Connected");

        }

        Err(e) => {

            println!("{:?}", e);

        }
    }
    }
}
}