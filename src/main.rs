extern crate libcomchannel;
extern crate libconfig;
extern crate libdb;
extern crate libdbgserver;
extern crate libengine;
extern crate libmercury;

use libcomchannel::*;
use libdb::DataBase;
use libdbgserver::debug_test;
use libengine::*;
use libmercury::device::*;
use libmercury::iface::*;
use std::path::*;
use std::sync::*;
use std::*;
use std::cell::RefCell;

fn main() {
    // Список интерфейсов-связи для создания
    let channels_registered: &[(&str, Box<RefCell<dyn ILinkChannelFactory>>)] =
        &mut [(SerialChannel::type_name(), Box::new(RefCell::new(LinkChannelFactory::default())))];

    let iface_registered: &[(&str, Box<dyn IFace>)] = &[(
        InterfaceMercury::type_name(),
        Box::new(InterfaceMercury::new()),
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
            if class_name == channel_classname.to_owned() {
                let mut channel = channel_factory.borrow_mut().spawn_with_uuid(guid.to_owned());
                let channel = Arc::new(Mutex::new(channel));
                channels_list.push(channel.clone());
            }
        }
        /*
        if class_name == SerialChannel::type_name() {
            let channel = Arc::new(Mutex::new(SerialChannel::new_with_uuid(guid.to_owned())));
            channels_list.push(channel);
        }
*/
        if class_name == InterfaceMercury::type_name() {
            let interface = Arc::new(Mutex::new(InterfaceMercury::new()));
            ifaces_list.push(interface);
        }
    }

    // Восстановление счётчиков
    let objects = db.load_objects();
    for obj in objects {
        let container = obj.unwrap();
        let guid = &container.guid;
        let class = &container.class;
        let parent = &container.parent;

        if class == Mercury230::type_name() {
            let _ = channels_list.iter_mut().map(|channel| {
                let arc_channel = channel.clone();
                let mut locked_channel = arc_channel.lock().unwrap();
                if parent == locked_channel.guid().as_str() {
                    let counter = Mercury230::new_with_uuid(guid.to_owned(), arc_channel.clone());
                    counters_list.push(Arc::new(Mutex::new(counter)));
                }
            });
        }
    }

    // Восстановление настроек
    // Активизация объектов

    // Ожидание комманд на отладочном сервере
    thread::spawn(move || {

        //engine::processing(request: &str)
    });

    debug_test();
}
