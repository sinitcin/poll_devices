
#[macro_use]
extern crate serde_json;

use serde_json::{Value, Error};


pub fn processing(request: &str) -> Result<String, Error> {
 
    let val: Value = serde_json::from_str(request)?;
   
    let action = match val["action"] {
        Value::String(ref expr) => expr,
        _ => "",
    };

    match action {
        "init" => {
            let respone = json!({
                "action" : "init",
                "code" : 200,
                "guid": "Тестовый GUID"
            });
            return Ok(respone.to_string());
        },
        _ => return Ok("No result!!!".to_string()),
    }; 
}


pub fn engine_test() {
  
    println!("{}", processing("{\"action\": \"init\"}").unwrap()); 
}
