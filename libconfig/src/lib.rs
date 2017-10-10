extern crate ini;

use ini::Ini;

#[derive(Debug, Default, Clone)]
pub struct Config<'a> {
    pub debug: Link<'a>,
    pub server: Link<'a>,
}

#[allow(dead_code)]
impl<'a> Config<'a> {

    pub fn init() -> Config<'a> {
        // Инициализация экземпляра данного менеджера
        let p = "cfg.ini";
        let cfg: Config = Config {
            debug: Link { base: "debug", path: p.clone()}, 
            server: Link {base: "server", path: p.clone()}
        };
        cfg
    }
}

#[derive(Debug, Default, Clone)]
pub struct Link<'b> {
    base: &'b str,
    path: &'b str,
}

#[allow(dead_code)]
impl<'b> Link<'b> {

    pub fn host(&self) -> Option<String> {
        // Чтение хоста      

        let conf = Ini::load_from_file(&self.path).unwrap();
        match conf.get_from(Some("links"), format!("host_{}", &self.base).as_ref()) {
            Some(param) => Some(param.to_owned()),
            None => None,
        }
    }

    pub fn set_host(&self, host: &str) {
        // Установка хоста
        
        let mut conf = Ini::load_from_file(&self.path).unwrap();
        conf.with_section(Some("links"))
            .set(format!("host_{}", &self.base).as_ref(), host);
        conf.write_to_file(&self.path).unwrap();
    }

    pub fn port(&self) -> Option<i32> {
        // Чтение порта 

        let conf = Ini::load_from_file(&self.path).unwrap();
        match conf.get_from(Some("links"), format!("port_{}", &self.base).as_ref()) {
            Some(param) => Some(param.parse().unwrap_or(8080i32)),
            None => None,
        }
    }

    pub fn set_port(&self, port: i32) {
        // Установка порта

        let mut conf = Ini::load_from_file(&self.path).unwrap();
        conf.with_section(Some("links"))
            .set(format!("port_{}", &self.base).as_ref(), format!("{}", port));
        conf.write_to_file(&self.path).unwrap();
    }
}

// #[cfg(test)]
pub mod tests {
    // #[test]
    pub fn config_test() {
        
        let cfg: super::Config = super::Config::init();
        
        // Настройка хоста отладки
        cfg.debug.set_host("127.0.0.2");
        assert_eq!(cfg.debug.host(), Some("127.0.0.2".to_owned()));

        // Настройка порта отладки
        cfg.debug.set_port(8181);
        assert_eq!(cfg.debug.port(), Some(8181));

        // Настройка хоста сервера
        cfg.server.set_host("127.0.0.3");
        assert_eq!(cfg.server.host(), Some("127.0.0.3".to_owned()));

        // Настройка порта сервера
        cfg.server.set_port(8282);
        assert_eq!(cfg.server.port(), Some(8282));
    }
}