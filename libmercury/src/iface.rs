extern crate libengine;
extern crate serial;

use iface::libengine::{IFace, StateLink, SerialConfig};
use self::serial::prelude::*;
use self::serial::BaudRate::Baud9600;

use std::time::Duration;

pub struct IFaceMercury230 {
    // Настройки COM-порта
    serial_cfg: SerialConfig,    
    // Состояние объекта
    suspended: bool,
    state: StateLink,
}

impl IFace for IFaceMercury230 {

    fn create() -> IFaceMercury230 {

        IFaceMercury230 {
            serial_cfg: SerialConfig {
                settings: {
                    baud_rate:    serial::Baud9600,
                    char_size:    serial::Bits8,
                    parity:       serial::ParityNone,
                    stop_bits:    serial::Stop1,
                    flow_control: serial::FlowNone,
                },
                port_name: "COM1".into_string(),
                timeout: 1,
                port: None,
            },
            suspended: true, 
            state: StateLink::Unknown,
       }
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