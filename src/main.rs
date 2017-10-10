extern crate libconfig;


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    #[test]
    fn config_test() {
        // Тестирование библиотеки libconfig
        libconfig::tests::config_test();
    }

    #[test]
    fn debug_test() {
        // Тестирование библиотеки libdbgserver
        libdbgserver::tests::debug_test();
    }
}