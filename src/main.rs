extern crate libconfig;
extern crate libdbgserver;
extern crate libengine;
extern crate libmercury;

use libdbgserver::debug_test;

fn collect_iface() {
    let ifaces = vec![libmercury::iface::IFaceMercury230::create()];
}

fn main() {
    println!("Hello!");
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