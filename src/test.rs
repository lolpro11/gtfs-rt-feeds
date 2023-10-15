extern crate serde;
extern crate serde_json;
mod dmfr;
use dmfr::Root;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = "/home/lolpro11/Documents/catenary-backend/transitland-atlas/feeds/";
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", entry.file_name().to_str().unwrap());
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let json = fs::read_to_string(&path)?;
            let feed: Root = serde_json::from_str(&json)?;
        }
    }
    Ok(())
}
