extern crate serde;
extern crate serde_json;
mod dmfr;
use dmfr::{DistributedMobilityFeedRegistry, FeedSpec};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = "/home/lolpro11/Documents/catenary-backend/transitland-atlas/feeds/";
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" && path.to_str().unwrap().contains("goswift.ly") {
            let json = fs::read_to_string(&path)?;
            let domain: DistributedMobilityFeedRegistry = serde_json::from_str(&json)?;
            for feed in domain.feeds {
                match feed.spec {
                    FeedSpec::GtfsRt => {
                        println!("{}", feed.id);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
