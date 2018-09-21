use libengine::engine::*;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::iter::Iterator;
use std::time::Duration;
use std::sync::Mutex;

struct SerialChannel {
    port: Option<ISerialPort>,
    port_name: String,
    baud_rate: serial::BaudRate,
    _child: Vec<Rc<RefCell<ICounter>>>,
}

impl ILinkChannel for SerialChannel {
    fn new() -> SerialChannel {
        SerialChannel {
            port: None,
            port_name: "COM1".to_owned(),
            baud_rate: serial::Baud9600,
            _child: vec![],
        }
    }

    fn reconf(&mut self) {
        self.port = Some(Rc::new(RefCell::new(
            serial::open(&self.port_name).unwrap(),
        )));

        let settings: serial::PortSettings = serial::PortSettings {
            baud_rate: self.baud_rate.clone(),
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone,
        };
        if let Some(ref mut port) = self.port {
            let _ = port.borrow_mut().configure(&settings).unwrap();
            port.borrow_mut()
                .set_timeout(Duration::from_secs(1))
                .unwrap();
        }
    }

    fn send(&mut self, data: &Vec<u8>) {
        if let Some(ref mut port) = self.port {
            let _ = port.borrow_mut().write(&data[..]).unwrap();
        }
    }

    fn read(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = (0..255).collect();

        if let Some(ref mut port) = self.port {
            match port.borrow_mut().read(&mut result[..]) {
                Ok(reading) => result.truncate(reading),
                Err(_) => result.truncate(0),
            }
        };
        result
    }
}