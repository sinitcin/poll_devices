use libengine::engine::*;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::iter::Iterator;
use std::sync::Mutex;

pub struct InterfaceMercury {
    // Состояние объекта
    guid: IGUID,
    suspended: bool,
    state: StateLink,
    counters: Vec<Box<Mutex<dyn ICounter + Send>>>,
}

impl IFace for InterfaceMercury {
    fn new() -> Self
    where
        Self: Sized,
    {
        InterfaceMercury {
            guid: String::new(),
            suspended: true,
            state: StateLink::Unknown,
            counters: vec![],
        }
    }

    fn new_with_uuid(uuid: IGUID) -> Self
    where
        Self: Sized,
    {
        InterfaceMercury {
            guid: uuid,
            suspended: true,
            state: StateLink::Unknown,
            counters: vec![],
        }
    }

    // Производим обмен со всеми счётчиками
    fn processing(&mut self) {
        let _ = self.counters.iter_mut().map(|counter| {
            if let Ok(mut counter_borrowed) = counter.lock() {
                counter_borrowed.communicate();
            }
        });
    }

    fn type_name() -> &'static str
    where
        Self: Sized,
    {
        "IFaceMercury230"
    }

    fn description() -> &'static str
    where
        Self: Sized,
    {
        "Меркурий 230"
    }
}
