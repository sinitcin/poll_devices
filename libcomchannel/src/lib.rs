use libengine::engine::*;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::iter::Iterator;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

#[derive(Default)]
pub struct LinkChannelFactory;

impl ILinkChannelFactory for LinkChannelFactory {
    fn spawn(&mut self) -> Arc<Mutex<dyn ILinkChannel>> {
        Arc::new(Mutex::new(SerialChannel {
            guid: String::new(),
            port: None,
            port_name: "COM1".to_owned(),
            baud_rate: serial::Baud9600,
            _child: vec![],
        }))
    }

    fn spawn_with_uuid(&mut self, uuid: IGUID) -> Arc<Mutex<dyn ILinkChannel>> {
        Arc::new(Mutex::new(SerialChannel {
            guid: uuid,
            port: None,
            port_name: "COM1".to_owned(),
            baud_rate: serial::Baud9600,
            _child: vec![],
        }))
    }
}

pub struct SerialChannel {
    guid: IGUID,
    port: Option<ISerialPort>,
    port_name: String,
    baud_rate: serial::BaudRate,
    _child: Vec<Rc<RefCell<ICounter>>>,
}

impl ILinkChannel for SerialChannel {
    fn guid(&mut self) -> IGUID {
        if self.guid.is_empty() {
            self.guid = format!("{}", Uuid::new_v4());
        }
        format!("{}", &self.guid)
    }

    fn reconf(&mut self) {
        self.port = Some(Arc::new(Mutex::new(serial::open(&self.port_name).unwrap())));

        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: self.baud_rate.clone(),
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };
        if let Some(ref mut port) = self.port {
            let arc_port = port.clone();
            let mut locked_port = arc_port.lock().unwrap();
            let _ = locked_port.configure(&settings).unwrap();
            locked_port.set_timeout(Duration::from_secs(1)).unwrap();
        }
    }

    fn send(&mut self, data: &Vec<u8>) {
        if let Some(ref mut port) = self.port {
            let _ = port.clone().lock().unwrap().write(&data[..]).unwrap();
        }
    }

    fn read(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = (0..255).collect();

        if let Some(ref mut port) = self.port {
            match port.clone().lock().unwrap().read(&mut result[..]) {
                Ok(reading) => result.truncate(reading),
                Err(_) => result.truncate(0),
            }
        };
        result
    }

    // Тип счётчика
    fn type_name() -> &'static str {
        "SerialChannel"
    }
}
