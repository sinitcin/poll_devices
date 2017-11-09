extern crate serde;
extern crate serde_json;

extern crate libengine;
extern crate serial;
extern crate serial_core;

use iface::libengine::{IFace, StateLink, SerialConfig};
use self::serial::prelude::*;

use std::time::Duration;

pub struct IFaceMercury230 {
    // Настройки COM-порта
    serial_cfg: SerialConfig,    
    // Состояние объекта
    suspended: bool,
    state: StateLink,
}

impl IFace for IFaceMercury230 {

    fn new() -> IFaceMercury230 {

        IFaceMercury230 {
            serial_cfg: SerialConfig {
                settings: serial::PortSettings {
                    baud_rate:    serial::BaudRate::Baud9600,
                    char_size:    serial::Bits8,
                    parity:       serial::ParityNone,
                    stop_bits:    serial::Stop1,
                    flow_control: serial::FlowNone,
                },
                port_name: "COM1".to_owned(),
                timeout: 1,
                port: None,
            },
            suspended: true, 
            state: StateLink::Unknown,
       }
    }

    fn type_name() ->  &'static str where Self: Sized {
        "IFaceMercury230"
    }

    fn description() ->  &'static str where Self: Sized {
        "Меркурий 230"
    }

    fn set_suspended(&mut self, pending: bool) {

        self.suspended = pending;
        if !pending {
            let mut port = Box::new(serial::open(&self.serial_cfg.port_name).unwrap());
            port.configure(&self.serial_cfg.settings).unwrap();
            port.set_timeout(Duration::from_secs(self.serial_cfg.timeout)).unwrap();
            self.serial_cfg.port = Some(port);
            self.state = StateLink::Unknown;
        } else {
            self.serial_cfg.port = None;
            self.state = StateLink::Deactive;
        }
    }

    fn suspended(&self) -> bool {
        self.suspended
    }
}

pub struct IFaceMercury200 {
    // Настройки COM-порта
    serial_cfg: SerialConfig,    
    // Состояние объекта
    suspended: bool,
    state: StateLink,
}

impl IFace for IFaceMercury200 {

    fn new() -> IFaceMercury200 {

        IFaceMercury200 {
            serial_cfg: SerialConfig {
                settings: serial::PortSettings {
                    baud_rate:    serial::BaudRate::Baud9600,
                    char_size:    serial::Bits8,
                    parity:       serial::ParityNone,
                    stop_bits:    serial::Stop1,
                    flow_control: serial::FlowNone,
                },
                port_name: "COM1".to_owned(),
                timeout: 1,
                port: None,
            },
            suspended: true, 
            state: StateLink::Unknown,
       }
    }

    fn type_name() ->  &'static str where Self: Sized {
        "IFaceMercury200"
    }

    fn description() ->  &'static str where Self: Sized {
        "Меркурий 200"
    }

    fn set_suspended(&mut self, pending: bool) {

        self.suspended = pending;
        if !pending {
            let mut port = Box::new(serial::open(&self.serial_cfg.port_name).unwrap());
            port.configure(&self.serial_cfg.settings).unwrap();
            port.set_timeout(Duration::from_secs(self.serial_cfg.timeout)).unwrap();
            self.serial_cfg.port = Some(port);
            self.state = StateLink::Unknown;
        } else {
            self.serial_cfg.port = None;
            self.state = StateLink::Deactive;
        }
    }

    fn suspended(&self) -> bool {
        self.suspended
    }
}
