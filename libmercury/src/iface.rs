use libengine::engine::*;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::iter::Iterator;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct FaceFactory;

impl IFaceFactory for FaceFactory {
    fn spawn(&mut self) -> Arc<Mutex<dyn IFace>> {
        Arc::new(Mutex::new(InterfaceMercury {
            guid: String::new(),
            suspended: true,
            state: StateLink::Unknown,
            counters: vec![],
        }))
    }
    fn spawn_with_uuid(&mut self, uuid: IGUID) -> Arc<Mutex<dyn IFace>> {
        Arc::new(Mutex::new(InterfaceMercury {
            guid: uuid,
            suspended: true,
            state: StateLink::Unknown,
            counters: vec![],
        }))
    }
}

#[allow(dead_code)]
pub struct InterfaceMercury {
    // Состояние объекта
    guid: IGUID,
    suspended: bool,
    state: StateLink,
    counters: Vec<Box<Mutex<dyn ICounter + Send>>>,
}

impl IFace for InterfaceMercury {
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
