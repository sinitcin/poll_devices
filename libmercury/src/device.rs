
#![feature(int_to_from_bytes)]

use libengine::engine::*;

use serde_json::{Value, Error};
use std::thread;
use byteorder::{BigEndian, ReadBytesExt};
use crc::crc32;
use serial::prelude::*;
use serial::SerialPort;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::io::Cursor;
use std::iter::Iterator;
use std::rc::Rc;
use std::time::Duration;
use uuid::Uuid;

struct Mercury230 {
    _parent: Rc<RefCell<ILinkChannel>>,
    _consumption: IConsumption,
    _serial: Option<String>,
    _name: Option<String>,
    guid: IGUID,
    address: u8,
}

impl ICounter for Mercury230 {
    // Конструктор
    fn new(channel: Rc<RefCell<ILinkChannel>>) -> Self {
        Mercury230 {
            _parent: channel,
            _consumption: 0.0,
            _serial: None,
            _name: None,
            guid: String::new(),
            address: 0,
        }
    }

    // Уникальный GUID устройства
    fn guid(&mut self) -> IGUID {
        if self.guid.is_empty() {
            self.guid = format!("{}", Uuid::new_v4());
        }
        format!("{}", &self.guid)
    }

    // Добавление в канал связи команд
    fn communicate(&mut self) {
        // Получаем канал связи для работы
        let parent = self.parent();
        let mut parent_borrowed = parent.borrow_mut();

        // Настройка соединения
        parent_borrowed.reconf();

        // Генерируем пакет для получения расхода
        let mut consumption = vec![self.address, 05, 00, 01];
        let my_crc = crc32::checksum_ieee(&consumption[..]).to_le_bytes();
        consumption.extend_from_slice(&my_crc[..]);

        // Отсылаем пакет, получаем ответ и обрабатываем
        parent_borrowed.send(&consumption);
        let response = parent_borrowed.read();

        self.processing(consumption, response);
    }

    // Обработка ответов
    fn processing(&mut self, request: Vec<u8>, response: Vec<u8>) {
        match (request[2], request[3]) {
            (5, 0) => {
                // Был запрос о расходе
                let tariff = request[4];
                let mut rdr = Cursor::new(vec![response[4], response[5], response[2], response[3]]);
                self._consumption = rdr.read_f64::<BigEndian>().unwrap() / 1000.0;
                println!(
                    "Тариф: {} - Расход: {}",
                    tariff, self._consumption
                );
            }
            _ => (),
        }
    }

    // Вернуть расход
    fn consumption(&self) -> IConsumption {
        self._consumption
    }

    // Тип счётчика
    fn type_name() -> &'static str {
        "IMercury230"
    }

    // Имя счётчика
    fn name(&self) -> Option<String> {
        self._name.clone()
    }

    // Серийный номер
    fn serial(&self) -> Option<String> {
        self._serial.clone()
    }

    // Выполнить поверку
    fn verification(&self) -> std::io::Result<()> {
        Ok(())
    }

    // Дата поверки
    fn last_verification_date(&self) -> Option<Duration> {
        None
    }

    // Как часто надо делать поверку
    fn verification_interval(&self) -> Option<Duration> {
        None
    }

    // Установим интервал между поверками
    fn set_verification_interval(&mut self, _interval: Duration) -> std::io::Result<()> {
        Ok(())
    }

    // Вернуть канал связи
    fn parent(&self) -> Rc<RefCell<ILinkChannel>> {
        self._parent.clone()
    }
}

impl IElectroCounter for Mercury230 {
    type Energy = f64;
    type Phase = i32;
    type Voltage = f32;

    // Активная энергия
    fn active_energy(&self, _phase: Self::Phase) -> Option<Self::Energy> {
        None
    }

    // Реактивная энергия
    fn reactive_energy(&self, _phase: Self::Phase) -> Option<Self::Energy> {
        None
    }

    // Действующие значения фазных токов
    fn voltage(&self, _phase: Self::Phase) -> Option<Self::Voltage> {
        None
    }

    // Частота сети
    fn frequencies(&self, _phase: Self::Phase) -> Option<i32> {
        None
    }
}