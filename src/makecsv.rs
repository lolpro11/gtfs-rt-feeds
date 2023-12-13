use std::env;


fn all() -> Result<(), Box<dyn Error>> {
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
                        if let Some(auth) = feed.authorization.clone() {
                            csv_str.push_str(",true");
                            csv_str.push_str(format!(",{}", auth.type_.to_string()).as_str());
                            if feed.authorization.unwrap().param_name.is_some() {
                                csv_str.push_str(format!(",{}", auth.param_name.unwrap()).as_str());
                            } else {
                                csv_str.push_str(",");
                            }
                            csv_str.push_str(",EXAMPLEKEY");
                        } else {    
                            csv_str.push_str(",false,,,");
                        }
                        csv_str.push_str(",3,");
                        println!("{}", csv_str);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn noauth() -> Result<(), Box<dyn Error>> {
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
                            csv_str.push_str(",false,,,,1,");
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

fn auth() -> Result<(), Box<dyn Error>> {
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
                        if feed.authorization.is_some() {
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
                            csv_str.push_str(",true");
                            csv_str.push_str(format!(",{}", feed.authorization.clone().unwrap().type_.to_string()).as_str());
                            csv_str.push_str(format!(",{}", feed.authorization.unwrap().param_name.unwrap()).as_str());
                            csv_str.push_str("EXAMPLEKEY,3,");
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

fn main() -> Result<(), Box<dyn Error>> {
    let feed_type = arguments::parse(std::env::args())
        .expect("Add --feeds <string>")
        .get::<String>("feeds");
    if feed_type == "all" {
        all();
    } else if feed == "noauth" {
        noauth();
    } else if feed == "auth" {
        auth();
    }
}