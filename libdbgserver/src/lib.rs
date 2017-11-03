extern crate libengine;
#[macro_use]
extern crate serde_json;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct DebugServer {
    host: String,
    port: i32,
}

impl DebugServer {
     
    pub fn start(&self) {
        let host = self.host.clone();
        let port  = self.port.clone();
        let (sender, receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();
         
        thread::spawn(move || {
            thread_start(format!("{}:{}", host, port), sender);
        });
        let _ = receiver.recv().unwrap();
    }
}    

fn thread_start(addr: String, sender: Sender<bool>) {
        
    let listener = TcpListener::bind(addr).unwrap();
    let _ = sender.send(true);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
            
        thread::spawn(move || {
            let mut request = [0; 512];
            let nbytes = stream.read(&mut request).unwrap();
            let response = libengine::processing( &(&*String::from_utf8_lossy( &request ))[0 .. nbytes] ).unwrap();
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
    }
}

pub fn debug_test() {
    let server: DebugServer = DebugServer{host: "127.0.0.1".to_string(), port: 8080};
    server.start();

    println!("Отправляем данные нашему серверу");
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Не могу подключиться к серверу...");
    let command = json!({
        "action": "init"
    });
    let _ = stream.write(command.to_string().as_bytes());

    println!("Получаем данные от него");
    let mut buffer = String::new();
    let _ = stream.read_to_string(&mut buffer).unwrap();

    println!("Проверка");
    assert_eq!(buffer, "{\"action\":\"init\",\"code\":200,\"guid\":\"Тестовый GUID\"}".to_string());
}
