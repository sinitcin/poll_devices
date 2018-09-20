#![feature(int_to_from_bytes)]
#![feature(extern_prelude)]

use serde_json::{Value, Error, from_str};
use serial::{PortSettings, SerialPort};
use std::cell::RefCell;
#[allow(unused_imports)]
use std::*;
use std::rc::Rc;
use std::time::Duration;

/// Состояние программы 
static mut PROGRAM_STATE: ProgramState = ProgramState::Starting;

/// Тип соответствует представлению последовательного порта
type ISerialPort = Rc<RefCell<SerialPort>>;

/// Тип представляет из себя UUID
pub type IGUID = String;

/// Расход счётчиков
type IConsumption = f64;

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
    pub settings: PortSettings,
    pub port_name: String,
    pub timeout: u64,
    pub port: Option<Box<SerialPort + Send>>,
}

/// # Типаж канала связи
///
pub trait ILinkChannel {
    /// Конструктор
    fn new() -> Self
    where
        Self: Sized;
    /// Настройка канала связи
    fn reconf(&mut self);
    /// Отправить данные
    fn send(&mut self, data: &Vec<u8>);
    /// Прочитать данные
    fn read(&mut self) -> Vec<u8>;
}

trait ICounter {
    /// Конструктор
    fn new(channel: Rc<RefCell<ILinkChannel>>) -> Self
    where
        Self: Sized;
    /// Уникальный GUID устройства
    fn guid(&mut self) -> IGUID;
    /// Добавление в канал связи команд
    fn communicate(&mut self);
    /// Обработка ответов
    fn processing(&mut self, request: Vec<u8>, response: Vec<u8>);
    /// Вернуть расход
    fn consumption(&self) -> IConsumption;
    /// Тип счётчика
    fn type_name() -> &'static str
    where
        Self: Sized;
    /// Имя счётчика
    fn name(&self) -> Option<String>;
    /// Серийный номер
    fn serial(&self) -> Option<String>;
    /// Выполнить поверку
    fn verification(&self) -> io::Result<()>;
    /// Дата поверки
    fn last_verification_date(&self) -> Option<Duration>;
    /// Как часто надо делать поверку
    fn verification_interval(&self) -> Option<Duration>;
    /// Установим интервал между поверками
    fn set_verification_interval(&mut self, interval: Duration) -> io::Result<()>;
    /// Вернуть канал связи
    fn parent(&self) -> Rc<RefCell<ILinkChannel>>;
}

trait IElectroCounter: ICounter {
    type Energy;
    type Phase;
    type Voltage;

    // Активная энергия
    fn active_energy(&self, phase: Self::Phase) -> Option<Self::Energy>;

    // Реактивная энергия
    fn reactive_energy(&self, phase: Self::Phase) -> Option<Self::Energy>;

    // Действующие значения фазных токов
    fn voltage(&self, phase: Self::Phase) -> Option<Self::Voltage>;

    // Частота сети
    fn frequencies(&self, phase: Self::Phase) -> Option<i32>;
}

///
/// Сетевой интерфейс, абстракция которая определяет как подключено любое устройство.
///
pub trait IFace: Send {

    // Создание экземпляра
    fn new() -> Self where Self: Sized;
    // Название класса
    fn type_name() ->  &'static str where Self: Sized;
    // Описание объекта
    fn description() ->  &'static str where Self: Sized;

    // Активен ли объект или спит?
    fn suspended(&self) -> bool;
    fn set_suspended(&mut self, pending: bool);
}

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
 
    let val: Value = from_str(request)?;
   
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
