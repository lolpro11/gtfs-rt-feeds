use std::fs;
use std::io::Read;
use serde_json::Value;
mod dmfr;
extern crate serde;
extern crate schemafy_core;
extern crate serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let dir_path = "/home/lolpro11/Documents/transitland-atlas/feeds";
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", entry.file_name().to_str().unwrap());
        if path.is_file() {
            let mut file = fs::File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            match serde_json::from_str::<Value>(&contents) {
                Ok(json) => {
                    if let Some(feeds) = json.get("feeds").and_then(Value::as_array) {
                        for feed in feeds {
                            if let Some(spec) = feed.get("spec").and_then(Value::as_str) {
                                println!("{}", spec);
                            }
                            if let Some(operators) = feed.get("operators").and_then(Value::as_array) {
                                for operator in operators {
                                    if let Some(onestop_id) = operator.get("onestop_id").and_then(Value::as_str) {
                                        println!("{}", onestop_id);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error deserializing JSON: {}", err);
                }
            }
        }
    }

    Ok(())
}
