use libengine::engine::*;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::iter::Iterator;
use std::time::Duration;

pub struct InterfaceMercury {
    // Состояние объекта
    suspended: bool,
    state: StateLink,
    counters: Vec<Box<RefCell<dyn ICounter>>>,
}

impl IFace for InterfaceMercury {
    fn new() -> InterfaceMercury {
        InterfaceMercury {
            suspended: true,
            state: StateLink::Unknown,
            counters: vec![],
        }
    }

    // Производим обмен со всеми счётчиками
    fn processing(&mut self) {
        let _ = self.counters.iter_mut().map(|counter| {
            if let Ok(mut counter_borrowed) = counter.try_borrow_mut() {
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
