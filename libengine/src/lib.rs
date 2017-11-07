extern crate serial;
#[macro_use]
extern crate serde_json;

use serde_json::{Value, Error};
use std::thread;
use serial::prelude::*;
use serial::SerialPort;
//use std::sync::{Arc, Mutex};

static mut PROGRAM_STATE: ProgramState = ProgramState::Starting;

type VCallBack = Option<Box<Fn() + Send>>;

/// Состояние программы в данный момент времени
enum ProgramState {
    /// Происходит запуск
    Starting, 
    /// Происходит закрытие
    Closing,
    /// Штатная работа
    Working,
}

/// Состояние линии связи
#[derive(Copy, Clone)]
pub enum StateLink {
    /// Не известное состояние связи, возможно происходит инициализация линии связи
    Unknown,
    /// Связь работает, данные проходят без повреждения
    Working,
    /// Нет соединения, данные не приходят и не уходят
    NoLink,
    /// Данные приходят поврежденные или не полные
    Corrupted, 
    /// Объект который осуществляет связь - не активен
    Deactive,
}

/// Параметры подключения
pub struct SerialConfig {
    pub settings: serial::PortSettings,
    pub port_name: String,
    pub timeout: u64,
    pub port: Option<Box<SerialPort + Send>>,
}

pub struct IFaceLink {
    __init: VCallBack,
    __free: VCallBack,
    __configure: VCallBack,
    __send: VCallBack,
    __recv: VCallBack,
    __do_session: VCallBack,
    __process_session: VCallBack,
    __post_session: VCallBack,
    __state: StateLink,
    __suspended: bool,
    serial_config: SerialConfig,
} 

///
/// Сетевой интерфейс, абстракция которая определяет как подключено любое устройство.
///
pub trait IFace: Send {

    /*
    /// Инициализация объекта
    fn init(&self);
    /// Финализация объекта
    fn free(&self);

    /// Настройка сетевого интерфейса
    fn configure(&self);
    /// Отправка данных
    fn send(&self);
    /// Получение данных
    fn recv(&self);
    /// Можно выполнить до начала сессии какие-то вещи
    fn do_session(&self);
    /// Сама сессия может быть запрограммирована
    fn process_session(&self);
    /// И после сессии
    fn post_session(&self);

    /// Состояние сети
    fn state(&self) -> StateLink;
    /// Установка состояние сети
    fn set_state(&mut self, state: StateLink);    

    */
    // Активен ли объект или спит?
    fn suspended(&self) -> bool;
    fn set_suspended(&mut self, pending: bool);
}

/// Реализация типажа IFace для объекта IFaceLink

/*
impl IFace for IFaceLink {
    fn init(&self) {
        if let Some(ref func) = self.__init {            
            func();
        }
    }

    fn free(&self) {
        if let Some(ref func) = self.__free {
            func();
        }
    }

    fn configure(&self) {
        if let Some(ref func) = self.__configure {
            func();
        }

    }

    fn send(&self) {
        if let Some(ref func) = self.__send {
            func();
        }

    }

    fn recv(&self) {
        if let Some(ref func) = self.__recv {
            func();
        }
    }

    fn do_session(&self) {
        if let Some(ref func) = self.__do_session {
            func();
        }
    }
    
    fn process_session(&self) {
        if let Some(ref func) = self.__process_session {
            func();
        }
    }

    fn post_session(&self) {
        if let Some(ref func) = self.__post_session {
            func();
        }
    }

    fn state(&self) -> StateLink {
        self.__state
    }

    fn set_state(&mut self, state: StateLink) {
        self.__state = state;
    }

    fn suspended(&self) -> bool {
        self.__suspended
    }

    fn set_suspended(&mut self, pending: bool) {
        self.__suspended = pending;
    }
}
*/

///
/// Определение устройств
///
trait Device {

}

///
/// Определение сенсоров
///
trait Sensor {

}

///
/// Определение счётчиков
///
trait Counter {

}


fn terminated() -> bool {
    // Узнать завершается ли программа
    unsafe {
        match PROGRAM_STATE {
            ProgramState::Closing => true,
            _ => false,
        }
    }
}
/*

fn poll(iface: Box<IFace>) {

    while !terminated() {
        iface.do_session();
        iface.process_session();
        iface.post_session();
    }
    iface.free();
}

///
/// Функция потока опроса устройств
///

pub fn polling(interfaces: Vec<Box<IFace>>) {

    for iface in interfaces {        
        iface.init();
        iface.configure();
        thread::spawn(move || {
            poll(iface);
        });
    }
}
*/
///
/// Обработка команд от сервера\клиента
///
pub fn processing(request: &str) -> Result<String, Error> {
 
    let val: Value = serde_json::from_str(request)?;
   
    let action = match val["action"] {
        Value::String(ref expr) => expr,
        _ => "",
    };

    match action {
        "init" => {
            let respone = json!({
                "action" : "init",
                "code" : 200,
                "guid": "Тестовый GUID"
            });
            return Ok(respone.to_string());
        },
        _ => return Ok("No result!!!".to_string()),
    }; 
}


pub fn engine_test() {
  
    println!("{}", processing("{\"action\": \"init\"}").unwrap()); 
}
