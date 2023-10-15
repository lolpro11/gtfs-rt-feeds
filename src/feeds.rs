extern crate serde;
extern crate serde_json;
mod dmfr;
use dmfr::Root;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let dir = "/home/lolpro11/Documents/catenary-backend/transitland-atlas/feeds/";
    println!("onestop,realtime_vehicle_positions,realtime_trip_updates,realtime_alerts,has_auth,auth_type,auth_header,auth_password,fetch_interval,multiauth");
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let mut csv_str  = "".to_string();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let json = fs::read_to_string(&path)?;
            let domain: Root = serde_json::from_str(&json)?;
            for feed in domain.feeds.unwrap() {
                if feed.spec == "gtfs-rt" {
                    csv_str.push_str(feed.id.as_str());
                    match feed.urls.realtime_vehicle_positions {
                        Some(realtime_vehicle_positions) => csv_str.push_str(format!(",{}", realtime_vehicle_positions.as_str()).as_str()),
                        None => csv_str.push_str(","),
                    }
                    match feed.urls.realtime_trip_updates {
                        Some(realtime_trip_updates) => csv_str.push_str(format!(",{}", realtime_trip_updates.as_str()).as_str()),
                        None => csv_str.push_str(","),
                    }
                    match feed.urls.realtime_alerts {
                        Some(realtime_alerts) => csv_str.push_str(format!(",{}", realtime_alerts.as_str()).as_str()),
                        None => csv_str.push_str(","),
                    }
                    if let Some(auth) = feed.authorization.clone() {
                        csv_str.push_str(",true");
                        csv_str.push_str(format!(",{}", auth.type_auth.as_str()).as_str());
                        if feed.authorization.unwrap().param_name.is_some() {
                            csv_str.push_str(format!(",{}", auth.param_name.unwrap().as_str()).as_str());
                        } else {
                            csv_str.push_str(",");
                        }
                    } else {    
                        csv_str.push_str(",false,,,");
                    }
                    csv_str.push_str(",3,\n");
                }
            }
        }
        println!("{}", csv_str);
    }
    Ok(())
}
