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
use std::cell::RefCell;
use std::path::*;
use std::sync::*;
use std::*;

fn main() {
    // Список интерфейсов-связи для создания
    let channels_registered: &[(&str, Box<RefCell<dyn ILinkChannelFactory>>)] = &mut [
        (
            SerialChannel::type_name(),
            Box::new(RefCell::new(LinkChannelFactory::default())),
        ),
        //(EthernetChannel::type_name(), Box::new(RefCell::new(EthernetChannelFactory::default()))),
        //(GSMChannel::type_name(), Box::new(RefCell::new(GSMChannelFactory::default()))),
    ];

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
        for counter in &counters_list {
            if *counter.lock().unwrap().guid() == container.guid {
                let mut properties = counter.lock().unwrap().properties().lock().unwrap();
                let mut item = PropertiesItem {
                    name: container.name.clone(),
                    value: container.value.clone(),
                    ptype: container.ptype,
                    variants: vec![],
                    regexpr: String::new(),
                    min: 0,
                    max: 999_999,
                    err_msg: String::new(),
                    required: container.required,
                };
                properties.add(container.name, item);
            }
        }
    }
    // Активизация объектов

    // Ожидание комманд на отладочном сервере
    thread::spawn(move || {

        //engine::processing(request: &str)
    });

    debug_test();
}
