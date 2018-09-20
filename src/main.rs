
extern crate libdbgserver;
extern crate libengine;
extern crate libmercury;
extern crate libdb;
extern crate libconfig;

use libmercury::iface::*;
use libdbgserver::debug_test;
use libengine::IFace;
use libdb::DataBase;
use std::path::*;

fn collect_iface() -> Vec<Box<IFace>> {
    let mercury230 = Box::new(InterfaceMercury::new());
    vec![mercury230]
}

fn main() {
    // Список интерфейсов-связи для создания
    let _registered: &[(&str, Box<IFace>)] = &[
                            (InterfaceMercury::type_name() , Box::new(InterfaceMercury::new())),
                        //    (IFaceMercury200::type_name() , Box::new(IFaceMercury200::new()))
                     ];

    // Чтение объектов из БД
    let mut db = DataBase::new();
    db.open(Path::new("debug.sqlite"));
    db.clear();
    let objects = db.load_objects();
    let mut ifaces: Vec<Box<IFace>> = Vec::new(); 

    // Восстановление интерфейсов
    for obj in objects {
        let class = obj.unwrap().class;

        if class == InterfaceMercury::type_name() {
            ifaces.push(Box::new(InterfaceMercury::new()));
        } else if class == InterfaceMercury::type_name() {
            ifaces.push(Box::new(InterfaceMercury::new()));
        }
    }

    let _ = collect_iface();
    debug_test();
}

#[cfg(test)]
mod tests {

    use libconfig::config_test;
    #[test]
    fn config() {
        // Тестирование библиотеки libconfig
        config_test();
    }

    use libdbgserver::debug_test;
    #[test]
    fn debug() {
        // Тестирование библиотеки libdbgserver
        debug_test();
    }

    use libengine::engine_test;
    #[test]
    fn engine() {
        engine_test();
    }
}