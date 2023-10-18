extern crate serde;
extern crate serde_json;
mod dmfr;
use dmfr::DistributedMobilityFeedRegistry;
use std::error::Error;
use std::fs;
use dmfr::FeedSpec;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = "/home/lolpro11/Documents/transitland-atlas/feeds/";
    println!("onestop,realtime_vehicle_positions,realtime_trip_updates,realtime_alerts,has_auth,auth_type,auth_header,auth_password,fetch_interval,multiauth");
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let json = fs::read_to_string(&path)?;
            let domain: DistributedMobilityFeedRegistry = serde_json::from_str(&json)?;
            for feed in domain.feeds {
                match feed.spec {
                    FeedSpec::GtfsRt => {//println!("{:?}", feed.clone());
                        if feed.authorization.is_none() {
                            let mut csv_str  = feed.id;
                            match feed.urls.realtime_vehicle_positions {
                                Some(realtime_vehicle_positions) => csv_str.push_str(format!(",{}", realtime_vehicle_positions.to_string()).as_str()),
                                None => csv_str.push_str(","),
                            }
                            match feed.urls.realtime_trip_updates {
                                Some(realtime_trip_updates) => csv_str.push_str(format!(",{}", realtime_trip_updates.to_string()).as_str()),
                                None => csv_str.push_str(","),
                            }
                            match feed.urls.realtime_alerts {
                                Some(realtime_alerts) => csv_str.push_str(format!(",{}", realtime_alerts.to_string()).as_str()),
                                None => csv_str.push_str(","),
                            }
                            csv_str.push_str(",false,,,,3,");
                            println!("{}", csv_str);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
