#Heading
Command line tool for parsing json data that I'm building to practice Rust.
---
## Heading
Inspired by jq (https://github.com/jqlang/jq).
---
### Heading
Usage:

  (As for now, you must have rust installed on your machine).
  1. Clone repository and use cargo to run the program.
  

### Heading
Command options:

    1. Displaying contents of a json file:   cargo run -- -file <file path>
    ---
    2. Displaying contents of a json file, but pretty:   cargo run -- -file <file path> -pretty
    ---
    3. Passing in json object directly written on command line:   cargo run -- -raw <json data>
    ---
    4. You can also use -pretty with this one:   cargo run -- -raw <json data> -pretty
    ---
    5. For now, for only one level of depth, extract the value of certain key:
          * cargo run -- -file <file path> -extract <key name>
          * Or
          * cargo run -- -raw <json data> -extract <key name>
    ---
    6. Or you can extract a dictionary of two keys given:
          * cargo run -- -file <file path> -extract <key name 1>,<key name 2> 
          * Or
          * cargo run -- -raw <json data> -extract <key name1>,<key name 2>

