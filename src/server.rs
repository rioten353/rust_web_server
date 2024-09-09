use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("server is running on {}", self.addr);

        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            println!("Received {} bytes", n);
                            println!("{}", String::from_utf8_lossy(&buffer[..n]));
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                    println!("client connected");
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}
