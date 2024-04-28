mod extractor;
mod scanner;

mod prelude {
    pub use crate::extractor::*;
    pub use crate::scanner::*;
    pub use serde_json::json;
    pub use std::{env, fs};
}


use prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let read_json: serde_json::Value = match args[1].as_str() {
        "-file" => from_file(&args[2]),

        "-raw" => serde_json::from_str(args[2].as_str())
            .expect("invalid json argument"),
        _ => panic!("Undefined argument"),
    };

    match env::args().nth(3) {
        None => {
            println!("{}", try_pretty(&read_json));
            std::process::exit(0);
        }
        Some(arg3) => {
            match query_handle(&arg3, &read_json) {
                Some(result) => println!("{}", result),
                None => println!("Invalid argument passed in")
            }
        }
    }

    std::process::exit(0);
}

fn query_handle(arg: &str, jsonv: &serde_json::Value) -> Option<serde_json::Value> {

    let mut scanner = Scanner::new(arg);

    if !scanner.take('.') {
        return None
    }

    let result = match scanner.peek() {
        Some('(') => {
            if let Some((l, r)) = Scanner::parse_pair::<String>(arg, ',') {
                let keys = (l.as_str(), r.as_str());
                Some(query_dict(&jsonv, keys))
            }
            else {
                None
            }
        },
        Some('{') => {
            // TODO: make this as in jq
            // field1: key1.key2.keyN, field2: key1.key2.keyN  }  
            //let mut content = scanner.take_until('}');
            // field1:key1.key2.keyN,...fieldN: key1.key2.keyN
            // TODO use a Map
            None
        },
        Some('[') => {
            // TODO: make this as in jq
            None
        },
        Some(_) => {
            let query_keys: Vec<&str> = arg.split_terminator('.')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();

            let result = match query_keys.len() {
                0 => jsonv.clone(),
                1 => query(jsonv, query_keys[0]),
                _ => query_nested(jsonv, query_keys),

            };

            return Some(result.clone());

        },

        None => None

    };

    result

}

fn from_file(name: &String) -> serde_json::Value {
    let string_json = fs::read_to_string(name).expect("File not found");

    let json: serde_json::Value =
        serde_json::from_str(string_json.as_str()).expect("json was not well formated");

    json
}

fn try_pretty(json_val: &serde_json::Value) -> String {
    match serde_json::to_string_pretty(json_val) {
        Ok(pretty_json) => pretty_json,
        Err(e) => format!("Failed at parsing json result: {:?}", e),
    }
}
