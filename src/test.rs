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
        let mut csv_str: String; //= entry.file_name().to_str().unwrap().to_string();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let json = fs::read_to_string(&path)?;
            let domain: Root = serde_json::from_str(&json)?;
            for feed in domain.feeds.unwrap() {
                //csv_str.push_str(format!(",{},{}", feed.id.as_str(), feed.spec.as_str()).as_str());
                if feed.spec == "gtfs-rt" {
                    if feed.authorization.is_some() {
                        println!("{},TRUE", feed.id);
                    } else {
                        println!("{},FALSE", feed.id);
                    }
                    
                }
            }
        }
        //println!("{}", csv_str);
    }
    Ok(())
}
