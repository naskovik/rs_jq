mod extractor;

mod prelude {
    pub use crate::extractor::*;
    pub use serde_json::json;
    pub use std::{env, fs};
}

use std::str::FromStr;

use prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let read_json: serde_json::Value = match args[1].as_str() {
        "-file" => from_file(&args[2]),

        "-raw" => serde_json::from_str(args[2].as_str()).expect("invalid json argument"),
        _ => panic!("Undefined argument"),
    };

    match env::args().nth(3) {
        None => {
            println!("{}", read_json.to_string());
            std::process::exit(0);
        }
        Some(arg3) => match arg3.as_str() {
            "-pretty" => {
                println!("{}", try_pretty(&read_json));
                std::process::exit(0);
            }
            "-extract" => {
                let extraction_argument =
                    env::args().nth(4).expect("-extract argument not provided");
                let json_value = match parse_pair::<String>(&extraction_argument, ',') {
                    Some((l, r)) => query_dict(&read_json, (l.as_str(), r.as_str())),
                    None => query(&read_json, &extraction_argument),
                };
                println!("{}", try_pretty(&json_value));
                std::process::exit(0);
            }
            _ => {
                println!("Unknown argument {}", args[3]);
            }
        },
    }
}

fn from_file(name: &String) -> serde_json::Value {
    let string_json = fs::read_to_string(name).expect("File not found");

    let json: serde_json::Value =
        serde_json::from_str(string_json.as_str()).expect("json was not well formated");

    json
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

fn try_pretty(json_val: &serde_json::Value) -> String {
    match serde_json::to_string_pretty(json_val) {
        Ok(pretty_json) => pretty_json,
        Err(e) => format!("Failed at parsing json result: {:?}", e),
    }
}
