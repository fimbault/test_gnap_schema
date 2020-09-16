use jsonschema::{CompilationError, Draft, JSONSchema};
use serde_json::{from_str, Value};
use std::{fs::File, io::Read, path::Path};

// read file
fn read_json(filepath: &str) -> Value {
    let path = Path::new(filepath);
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).ok().unwrap();
    let data: Value = from_str(&content).unwrap();
    data
}

// main
fn main() -> Result<(), CompilationError> {

    let schema = read_json("schema.json");
    let instance = read_json("test_invalid_resource.json");
    
    let compiled = JSONSchema::options()
         .with_draft(Draft::Draft7)
         .compile(&schema)?;
    
    let result = compiled.validate(&instance);
    
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error)
        }
    }

    Ok(())
}
