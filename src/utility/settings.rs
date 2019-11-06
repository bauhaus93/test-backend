use std::collections::HashMap;
use config::{Config, File};

lazy_static! {
    static ref SETTINGS: HashMap<String, String> = {
        let mut settings = Config::default();
        match settings.merge(File::with_name("settings")) {
            Ok(_) => info!("Loaded settings file"),
            Err(e) => warn!("Could not load settings: {}", e),
        }
        match settings.try_into::<HashMap<String, String>>() {
            Ok(hm) => {
                info!("Successfully read settings");
                for (k, v) in &hm {
                    info!("Key: {}, Value: {}", k, v);
                }
                hm
            },
            Err(e) => {
                error!("Could not transform settings into hashmap: {}", e);
                HashMap::default()
            }
        }
    };
}

pub fn get_setting(key: &str) -> String {
    match SETTINGS.get(&key.to_lowercase()) {
        Some(ip) => ip.to_owned(),
        None => {
            error!("'{}' not found in settings", key);
            "".to_owned()
        }
    }
}
