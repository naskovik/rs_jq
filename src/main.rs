mod extractor;
mod scanner;

mod prelude {
    pub use crate::extractor::*;
    pub use crate::scanner::*;
    pub use serde_json::json;
    pub use std::{env, fs, collections::HashMap};
}

use std::str::FromStr;

use prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct ParseModeError;
enum Mode {
    File,
    Raw
}

impl FromStr for Mode {
    type Err = ParseModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-file" => Ok(Mode::File),
            "-raw" => Ok(Mode::Raw),
            _ => Err(ParseModeError)
        }
    }

}

struct Arguments {
    pub mode: Result<Mode, ParseModeError>,
    pub json_arg: String,
    pub query_arg: Option<String>
}

impl Arguments {
    pub fn new(args: Vec<String>) -> Self {
        assert!(args.len() >= 2);
        let mode = Mode::from_str(args.iter().nth(1).unwrap().as_str());
        let json_arg = args.iter().nth(2).unwrap().to_string();
        let query_arg = match args.iter().nth(3) {
            Some(q) => Some(q).cloned(),
            None => None
        };
        Arguments {
            mode,
            json_arg,
            query_arg
        }
    }
}


fn main() {
    println!("{:?}", env::args().collect::<Vec<String>>());
    let args: Arguments = Arguments::new(env::args().collect());

    let read_json: serde_json::Value = match args.mode {
        Ok(Mode::File) => {
            let string_json = fs::read_to_string(args.json_arg).expect("File not found");

            let json: serde_json::Value =
            serde_json::from_str(string_json.as_str()).expect("json was not well formated");

            json
        },

        Ok(Mode::Raw) => serde_json::from_str(&args.json_arg)
            .expect("invalid json argument"),
        _ => panic!("Undefined argument"),
    };

    match args.query_arg {
        None => {
            println!("{}", try_pretty(&read_json));
            std::process::exit(0);
        }
        Some(query_param) => {
            match query_handle(&query_param, &read_json) {
                Some(result) => println!("{}", try_pretty(&result)),
                None => {
                    println!("Invalid argument passed in");
                    println!("{:?}", query_param)
                }
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


    let result = match scanner.pop() {
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
                match Scanner::parse_pair::<String>(&elem, ':') {
                    None => {},
                    Some((custom_key, query_keys)) => {
                        let keys_vec = Scanner::split_by(&query_keys, '.')
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
            let query_keys: Vec<String> = Scanner::split_by(arg, '.')?;

            let result = match query_keys.len() {
                0 => jsonv.clone(),
                _ => query(jsonv, query_keys),

            };

            Some(result.clone())

        },

        None => None

    };

    result

}

fn try_pretty(json_val: &serde_json::Value) -> String {
    match serde_json::to_string_pretty(json_val) {
        Ok(pretty_json) => pretty_json,
        Err(e) => format!("Failed at try_pretty: {:?} \n Showing not prettified:\n {}", e, json_val),
    }
}
