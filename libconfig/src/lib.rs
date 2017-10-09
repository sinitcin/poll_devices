extern crate ini;

use ini::Ini;

#[derive(Debug)]
pub struct Config {
    debug: Link,
}

#[allow(dead_code)]
impl Config {
    fn init(&mut self) {
        self.debug.base = "debug".to_owned();
        self.debug.path = "cfg.ini".to_owned();
    }
}

#[derive(Debug)]
struct Link {
    base: String,
    path: String,
}

#[allow(dead_code)]
impl Link {

    fn host(&self) -> Option<String> {
        // Чтение хоста      

        let conf = Ini::load_from_file(&self.path).unwrap();
        match conf.get_from(Some("links"), format!("host_{}", &self.base).as_ref()) {
            Some(param) => Some(param.to_owned()),
            None => None,
        }
    }

    fn set_host(&self, host: &str) {
        // Установка хоста
        
        let mut conf = Ini::new();
        conf.with_section(Some("links"))
            .set(format!("host_{}", &self.base).as_ref(), host);
        conf.write_to_file(&self.path).unwrap();
    }

    fn port(&self) -> Option<i32> {
        // Чтение порта 

        let conf = Ini::load_from_file(&self.path).unwrap();
        match conf.get_from(Some("links"), format!("port_{}", &self.base).as_ref()) {
            Some(param) => Some(param.parse().unwrap_or(8080)),
            None => None,
        }
    }

    fn set_port(&self, port: i32) {

        let mut conf = Ini::new();
        conf.with_section(Some("links"))
            .set(format!("port_{}", &self.base).as_ref(), format!("{}", port));
        conf.write_to_file(&self.path).unwrap();
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let cfg: config;
        cfg.init();
        cfg.set_host("127.0.0.2");
        assert!(cfg.host(), "127.0.0.2");
        
   }
}