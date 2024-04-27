
### Command line tool for parsing json data that I'm building to practice Rust.
---

#### Inspired by jq (https://github.com/jqlang/jq).
---

#### Usage:

  (As for now, you must have rust installed on your machine).
  1. Clone repository and use cargo to run the program.
  


#### Command options:

**Reading will prettify the passed in contents by default:**

        cargo run -- -file <file path>
        cargo run -- -raw <json data>    
        
**To query fields from the json**

        cargo run -- -file <file path> .<key names connected by dots>
        
**Or you can extract a dictionary of two keys given:**

        cargo run -- -file <file path> .(<key name 1>,<key name 2>) 
        cargo run -- -raw <json data> .(<key name 1>,<key name 2>)
        
---        

