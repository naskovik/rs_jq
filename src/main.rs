mod extractor;
mod scanner;

mod prelude {
    pub use crate::extractor::*;
    pub use crate::scanner::*;
    pub use serde_json::json;
    pub use std::{env, fs, collections::HashMap};
}

use prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let read_json: serde_json::Value = match args[1].as_str() {
        "-file" => {
            let string_json = fs::read_to_string(&args[2]).expect("File not found");

            let json: serde_json::Value =
            serde_json::from_str(string_json.as_str()).expect("json was not well formated");

            json
        },

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
            // field1: key1.key2.keyN, field2: key1.key2.keyN  }  
            let content = scanner.take_until('}')?;
            // field1:key1.key2.keyN,...fieldN: key1.key2.keyN
            let lvl1 = Scanner::split_by(&content, ',')?;

            let mut keys_set: HashMap<String, Vec<String>> = HashMap::new();
            lvl1.into_iter().for_each(|elem| {
                match Scanner::parse_pair::<String>(elem, ':') {
                    None => {},
                    Some((custom_key, query_keys)) => {
                        let keys_vec = Scanner::split_by_noref(query_keys, '.')
                            .unwrap_or(vec![]);
                        keys_set.insert(custom_key, keys_vec);
                    }
                }
        
            });
            

            Some(query_for_custom(&jsonv, keys_set))
        },
        Some('[') => {
            let content = scanner.take_until(']')?;
            if let Ok(index) = content.parse::<usize>() {
                Some(query_from_vec_w_index(&jsonv, index))
            } else {
                Some(serde_json::Value::default())
            }
            
        },
        Some(_) => {
            let query_keys: Vec<String> = Scanner::split_by_noref(arg.to_string(), '.')?;

            let result = match query_keys.len() {
                0 => jsonv.clone(),
                _ => query(jsonv, query_keys),

            };

            return Some(result.clone());

        },

        None => None

    };

    result

}

fn try_pretty(json_val: &serde_json::Value) -> String {
    match serde_json::to_string_pretty(json_val) {
        Ok(pretty_json) => pretty_json,
        Err(e) => format!("Failed at parsing json result: {:?}", e),
    }
}
