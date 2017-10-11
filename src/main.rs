extern crate libconfig;
extern crate libdbgserver;


fn main() {
    println!("Hello, world!");
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
}