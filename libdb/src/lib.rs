extern crate rusqlite;

use std::path::Path;
use rusqlite::Connection;

///Перечисление возможных статусов возврата 
///
#[derive(Debug)]
pub enum DBStatus {    
    /// Операция выполнена успешно
    Ok, 
    /// Отсутствует БД
    DBMissing,
    /// БД заблокирована другим процессом
    DBBussy,
}

#[derive(Debug)]
pub struct DataBase;

impl DataBase {

    /// Открытие БД
    fn open(path: &Path) -> DBStatus {

        DBStatus::Ok
    }

    /// Очистка БД
    fn clear() -> DBStatus {

        DBStatus::Ok
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
