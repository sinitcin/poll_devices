use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub struct DebugServer {
    host: String,
    port: i32,
}

impl DebugServer {
     
    pub fn start(&self) {
        let host = self.host.clone();
        let port  = self.port.clone();
         
        thread::spawn(move || {
            thread_start(format!("{}:{}", host, port));
        });                
    }
}    

fn thread_start(addr: String) {
        
    let listener = TcpListener::bind(addr).unwrap();        
    for stream in listener.incoming() {
        let stream = stream.unwrap();
            
        thread::spawn(move || {
            handle_connection(stream.try_clone().unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

pub fn debug_test() {
    let server: DebugServer = DebugServer{host: "127.0.0.1".to_owned(), port: 8080};
    server.start();
    thread::sleep(Duration::from_millis(60000))
}
