use std::net::{ TcpListener, TcpStream};
use std::io::{ Read, BufReader };

pub struct Server {
    address: String,
    listener: TcpListener,
    connections: Vec<TcpStream>
}

impl Server {
    pub fn new(address: String) -> Self {
        let listener = TcpListener::bind(address.as_str()).unwrap();
        Self {
            address,
            listener,
            connections: vec![]
        }
    }

    pub fn listen(&mut self) {
        for stream in self.connection.incoming() {
            match stream {
                Ok(mut stream) => {
                    let peer = stream.peer_addr().unwrap();
                    let local = stream.local_addr().unwrap();
                    println!("new client! \npeer: {}\nlocal: {}\n", peer, local);
                    self.connections.push(stream);
//                    let mut buf = [0; 1];
//                    loop {
//                        stream.read_exact(&mut buf);
//                        let sign = format!("{}", String::from_utf8_lossy(&buf));
//                        if !sign.is_empty() && sign != "\n" {
//                            print!("\r{:?}", sign);
//                        }
//                    }
                }
                Err(e) => { println!("connection failed"); }
            }
        }
    }
}


pub struct Client {
    address: String,
    connection: _,
}

impl Client {
    pub fn new(address: String) -> Self {
        let connection = TcpListener::bind(address.as_str()).unwrap();
        Self {
            address,
            connection,
        }
    }

    pub fn send
}
