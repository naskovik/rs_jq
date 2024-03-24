
# Command line tool for parsing json data that I'm building to practice Rust.
---

## Inspired by jq (https://github.com/jqlang/jq).
---

### Usage:

  (As for now, you must have rust installed on your machine).
  1. Clone repository and use cargo to run the program.
  


### Command options:

**Displaying contents of a json file:**

        ```cargo run -- -file <file path>```
    
**Displaying contents of a json file, but pretty:**

        ```cargo run -- -file <file path> -pretty```
    
**Passing in json object directly written on command line:**

        ```cargo run -- -raw <json data>```
    
**You can also use -pretty with this one:**

        ```cargo run -- -raw <json data> -pretty```
    
**For now, for only one level of depth, extract the value of certain key:**

        ```cargo run -- -file <file path> -extract <key name>```
        
          **Or**
        
        ```cargo run -- -raw <json data> -extract <key name>```
    
**Or you can extract a dictionary of two keys given:**

        ```cargo run -- -file <file path> -extract <key name 1>,<key name 2>``` 
        
          **Or**
        
        ```cargo run -- -raw <json data> -extract <key name 1>,<key name 2>```


---        

