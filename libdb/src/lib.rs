extern crate rusqlite;

use rusqlite::Connection;
use std::path::Path;

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
pub struct DataBaseObject {
    pub guid: String,
    pub parent: String,
    pub class: String,
    pub created: f64,
}

#[derive(Debug)]
pub struct DataBaseProperties {
    pub guid: String,
    pub name: String,
    pub value: String,
    pub ptype: i8,
    pub required: bool,
}

#[derive(Debug)]
pub struct DataBase {
    conn: Option<Connection>,
}

impl DataBase {
    /// Конструктор
    pub fn new() -> DataBase {
        DataBase { conn: None }
    }

    /// Открытие БД
    pub fn open(&mut self, path: &Path) -> DBStatus {
        self.conn = Some(Connection::open(path).unwrap());
        DBStatus::Ok
    }

    /// Загрузка объектов из БД
    pub fn load_objects(&self) -> Vec<Result<DataBaseObject, rusqlite::Error>> {
        let mut result = Vec::new();
        if let Some(ref conn) = self.conn {
            let mut stmt = conn
                .prepare("SELECT guid, parent, class, created FROM objects")
                .unwrap();
            let obj_iter = stmt
                .query_map(&[], |row| DataBaseObject {
                    guid: row.get(0),
                    parent: row.get(1),
                    class: row.get(2),
                    created: row.get(3),
                })
                .unwrap();
            result = obj_iter.collect();
        }
        result
    }

    // Создать запись о новом объекте
    pub fn store_object(&self, obj: &DataBaseObject) {
        if let Some(ref conn) = self.conn {
            conn.execute(
                "INSERT INTO objects (guid, parent, class, created) VALUES (?1, ?2, ?3, ?4)",
                &[&obj.guid, &obj.parent, &obj.class, &obj.created],
            )
            .unwrap();
        }
    }

    /// Загрузка свойств объектов системы
    pub fn load_properties(&self) -> Vec<Result<DataBaseProperties, rusqlite::Error>> {
        let mut result = Vec::new();
        if let Some(ref conn) = self.conn {
            let mut stmt = conn
                .prepare("SELECT guid, name, value FROM PROPERTIES")
                .unwrap();
            let obj_iter = stmt
                .query_map(&[], |row| DataBaseProperties {
                    guid: row.get(0),
                    name: row.get(1),
                    value: row.get(2),
                    ptype: row.get(3),
                    required: row.get(4),
                })
                .unwrap();
            result = obj_iter.collect();
        }
        result
    }

    /// Очистка БД
    pub fn clear(&self) -> DBStatus {
        if let Some(ref conn) = self.conn {
            if let Err(err) = conn.execute("DROP TABLE OBJECTS", &[]) {
                println!("database failed => {}", err);
            }
            if let Err(err) = conn.execute("DROP TABLE PROPERTIES", &[]) {
                println!("database failed => {}", err);
            }
            conn.execute(
                "CREATE TABLE `OBJECTS` (
                            	`GUID`	    TEXT NOT NULL UNIQUE,
	                            `PARENT`	TEXT NOT NULL,
	                            `CLASS`	    TEXT NOT NULL,
	                            `CREATED`	DOUBLE );",
                &[],
            )
            .unwrap();
            conn.execute(
                "CREATE TABLE `PROPERTIES` (
                            	`GUID`	    TEXT NOT NULL UNIQUE,
	                            `NAME`	    TEXT NOT NULL,
	                            `VALUE`	    TEXT,
                                `PTYPE`     INTEGER,
                                `REQUIRED`  BOOLEAN
                                );",
                &[],
            )
            .unwrap();
        }
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
