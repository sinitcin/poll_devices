use serde_json::{from_str, Error, Value};
use serial::{PortSettings, SerialPort};
#[allow(unused_imports)]
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;
#[allow(unused_imports)]
use std::*;

/// Состояние программы
static mut PROGRAM_STATE: ProgramState = ProgramState::Starting;

/// Тип соответствует представлению последовательного порта
pub type ISerialPort = Arc<Mutex<SerialPort>>;

/// Тип представляет из себя UUID
pub type IGUID = String;

/// Расход счётчиков
pub type IConsumption = f64;

/// Состояние программы в данный момент времени
pub enum ProgramState {
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

/// Общие данные всех системных объектов
pub trait IGeneralInformation {
    /// Уникальный GUID устройства
    fn guid(&mut self) -> IGUID;
    // Определение типа для динамической диспетчеризации
    fn type_name() -> &'static str
    where
        Self: Sized;
    // Описание объекта
    fn description() -> &'static str
    where
        Self: Sized;
}

/// Макросы для создания менеджера свойств
#[macro_export]
macro_rules! propertie_manager {
    ($factory:expr, $manager:expr) => {

        #[derive(Default)]
        pub struct $factory;

        impl IManagerPropertiesFactory for $factory {
            fn spawn() -> Arc<Mutex<dyn IManagerProperties>> {
                Arc::new(Mutex::new($manager {list: HashMap::new()}))
            }
        }
        
        pub struct $manager {
            list: HashMap<String, PropertiesItem>,
        }

        impl IManagerProperties for $manager  {

            fn add(&mut self, item: PropertiesItem) {
                &self.list.insert(item.name.clone(), item);
            }

            fn set_value_by_name(&self, name: &str, value: &str) {

            }

            fn set_value_by_index(&self, index: i32, value: &str) {

            }

            fn list_properties(&self) -> Vec<&PropertiesItem> {
                let mut result = vec![];
                for value in self.list.values() {
                    result.push(value);
                }
                result
            }
        }
    };
}


/// Фабрика по созданию каналов связи
pub trait ILinkChannelFactory {
    fn spawn(&mut self) -> Arc<Mutex<dyn ILinkChannel>>;
    fn spawn_with_uuid(&mut self, uuid: IGUID) -> Arc<Mutex<dyn ILinkChannel>>;
}

/// # Типаж канала связи
///
pub trait ILinkChannel {
    /// Уникальный GUID устройства
    fn guid(&mut self) -> IGUID;
    /// Настройка канала связи
    fn reconf(&mut self);
    /// Отправить данные
    fn send(&mut self, data: &Vec<u8>);
    /// Прочитать данные
    fn read(&mut self) -> Vec<u8>;
    // Определение типа
    fn type_name() -> &'static str
    where
        Self: Sized;
    /// Настраиваемые свойства объекта
    fn properties(&self) -> Arc<Mutex<IManagerProperties>>;
}

/// Фабрика по созданию счётчиков
pub trait ICounterFactory {
    fn spawn(&mut self, channel: Arc<Mutex<ILinkChannel>>) -> Arc<Mutex<dyn ICounter>>;
    fn spawn_with_uuid(
        &mut self,
        uuid: IGUID,
        channel: Arc<Mutex<ILinkChannel>>,
    ) -> Arc<Mutex<dyn ICounter>>;
}

pub trait ICounter {
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
    fn parent(&self) -> Arc<Mutex<ILinkChannel>>;
    /// Настраиваемые свойства объекта
    fn properties(&self) -> Arc<Mutex<IManagerProperties>>;
}

pub trait IElectroCounter: ICounter {
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

// Фабрика по созданию интерфейсов
pub trait IFaceFactory {
    fn spawn(&mut self) -> Arc<Mutex<dyn IFace>>;
    fn spawn_with_uuid(&mut self, uuid: IGUID) -> Arc<Mutex<dyn IFace>>;
}

/// Сетевой интерфейс, абстракция которая определяет как подключено любое устройство.
pub trait IFace: Send {
    // Обмен со всеми дочерними устройствами
    fn processing(&mut self);
    // Название класса
    fn type_name() -> &'static str
    where
        Self: Sized;
    // Описание объекта
    fn description() -> &'static str
    where
        Self: Sized;
}

// Фабрика для менеджеров свойств
pub trait IManagerPropertiesFactory {
    fn spawn() -> Arc<Mutex<dyn IManagerProperties>>;
}

// Типаж менеджера свойств
pub trait IManagerProperties {
    fn add(&mut self, item: PropertiesItem);
    fn set_value_by_name(&self, name: &str, value: &str);
    fn set_value_by_index(&self, index: i32, value: &str);
    fn list_properties(&self) -> Vec<&PropertiesItem>;
}

#[allow(dead_code)]
// Тип каждого свойства
pub enum PropertiesType {
    Read,
    ReadWrite,
    Hide,
}

#[allow(dead_code)]
// Каждое свойство в менеджере свойств является структурой
pub struct PropertiesItem {
    pub name: String,
    pub value: String,
    pub ptype: PropertiesType,
    pub variants: Vec<String>,
    pub regexpr: String,
    pub min: i16,
    pub max: i16,
    pub err_msg: String,
    pub required: bool,
}

#[allow(dead_code)]
fn terminated() -> bool {
    // Узнать завершается ли программа
    unsafe {
        match PROGRAM_STATE {
            ProgramState::Closing => true,
            _ => false,
        }
    }
}

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
        }
        _ => return Ok("No result!!!".to_string()),
    };
}

pub fn engine_test() {
    println!("{}", processing("{\"action\": \"init\"}").unwrap());
}
