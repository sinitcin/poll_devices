extern crate libcomchannel;
extern crate libconfig;
extern crate libdb;
extern crate libdbgserver;
extern crate libengine;
extern crate libmercury;

use libcomchannel::*;
use libdb::DataBase;
use libengine::*;
use libmercury::device::*;
use libmercury::iface::*;
use std::cell::RefCell;
use std::io::prelude::*;
use std::io::{BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::*;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    // Список интерфейсов-связи для создания
    let channels_registered: &[(&str, Box<RefCell<dyn ILinkChannelFactory>>)] = &mut [(
        SerialChannel::type_name(),
        Box::new(RefCell::new(LinkChannelFactory::default())),
    )];

    let iface_registered: &[(&str, Box<RefCell<dyn IFaceFactory>>)] = &mut [(
        InterfaceMercury::type_name(),
        Box::new(RefCell::new(FaceFactory::default())),
    )];

    let counter_registered: &[(&str, Box<RefCell<dyn ICounterFactory>>)] = &mut [(
        InterfaceMercury::type_name(),
        Box::new(RefCell::new(Mercury230Factory::default())),
    )];

    // Чтение объектов из БД
    let mut db = DataBase::new();
    db.open(Path::new("debug.sqlite"));
    db.clear();
    let mut channels_list: Vec<Arc<Mutex<dyn ILinkChannel>>> = Vec::new();
    let mut ifaces_list: Vec<Arc<Mutex<IFace>>> = Vec::new();
    let mut counters_list: Vec<Arc<Mutex<ICounter>>> = Vec::new();

    // Восстановление каналов и интерфейсов
    let objects = db.load_objects();
    for obj in objects {
        let container = obj.unwrap();
        let guid = &container.guid;
        let class_name = &container.class;

        for channel_reg in channels_registered {
            let (channel_classname, channel_factory) = channel_reg;
            if *class_name == channel_classname.to_owned() {
                let mut channel = channel_factory
                    .borrow_mut()
                    .spawn_with_uuid(guid.to_owned());
                channels_list.push(channel);
            }
        }

        for iface_reg in iface_registered {
            let (iface_classname, iface_factory) = iface_reg;
            if *class_name == iface_classname.to_owned() {
                let mut iface = iface_factory.borrow_mut().spawn_with_uuid(guid.to_owned());
                ifaces_list.push(iface);
            }
        }
    }

    // Восстановление счётчиков
    let objects = db.load_objects();
    for obj in objects {
        let container = obj.unwrap();
        let guid = &container.guid;
        let class_name = &container.class;
        let parent = &container.parent;

        for counter_reg in counter_registered {
            let (counter_classname, counter_factory) = counter_reg;
            if *class_name == counter_classname.to_owned() {
                let _ = channels_list.iter_mut().map(|channel| {
                    let arc_channel = channel.clone();
                    let mut locked_channel = arc_channel.lock().unwrap();
                    if parent == locked_channel.guid().as_str() {
                        let counter = counter_factory
                            .borrow_mut()
                            .spawn_with_uuid(guid.to_owned(), arc_channel.clone());
                        &counters_list.push(counter);
                    };
                });
            }
        }
    }

    // Восстановление настроек
    let rows = db.load_properties();
    for row in rows {
        let container = row.unwrap();
        if &container.name == "Активность" {
            continue;
        }
        for counter in &counters_list {
            if *counter.lock().unwrap().guid() == container.guid {
                let mut properties = counter.lock().unwrap().properties();
                let mut properties = properties.lock().unwrap();
                let mut item = PropertiesItem {
                    name: container.name.clone(),
                    value: container.value.clone(),
                    ptype: container.ptype.into(),
                    variants: vec![],
                    regexpr: String::new(),
                    min: 0,
                    max: 32_767,
                    err_msg: String::new(),
                    required: container.required,
                };
                properties.add(item);
            }
        }
    }

    // Активизация объектов
    let rows = db.load_properties();
    for row in rows {
        let container = row.unwrap();
        if &container.name != "Активность" {
            continue;
        }
        for counter in &counters_list {
            if *counter.lock().unwrap().guid() == container.guid {
                let mut properties = counter.lock().unwrap().properties();
                let mut properties = properties.lock().unwrap();
                let mut item = PropertiesItem {
                    name: container.name.clone(),
                    value: container.value.clone(),
                    ptype: container.ptype.into(),
                    variants: vec![],
                    regexpr: String::new(),
                    min: 0,
                    max: 32_767,
                    err_msg: String::new(),
                    required: container.required,
                };
                properties.add(item);
            }
        }
    }

    // Ожидание комманд на отладочном сервере
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = String::new();
    {
        let mut reader = BufReader::new(&mut stream);
        reader.read_line(&mut buffer).unwrap();
        println!("{}", &buffer);
    }

    let response = engine::processing(&buffer.as_str()).unwrap_or("{error}".to_string());

    println!("{}", &response);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("End");
}
