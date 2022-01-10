use ron::de::from_str;
use serde::Deserialize;
use std::{fs::read_to_string, ops::Range};

const MAP_SETTINGS_FILE_NAME: &str = "map_settings.ron";

#[derive(Debug, Deserialize)]
pub struct MapGenSettings {
    pub seed: u64,
    pub iterations: u32,
    pub map_size: [u32;2],
    pub room_size: Range<u32>,
    pub monsters_per_room: Range<u32>,
    //pub items_per_room: Range<u32>,
}

impl Default for MapGenSettings {
    fn default() -> Self {
        Self {
            seed: 5,
            iterations: 15,
            map_size: [80, 40],
            room_size: 3..15,
            monsters_per_room: 0..4,
            //items_per_room: 0..2,
        }
    }
}

pub fn try_get_map_settings() -> Result<MapGenSettings, String> {
    let result = read_to_string(format!(
        "{}/assets/{}",
        env!("CARGO_MANIFEST_DIR"),
        MAP_SETTINGS_FILE_NAME
    ));

    let file_string = match result {
        Ok(file_string) => file_string,
        Err(_) => return Err(format!("Error reading {}", MAP_SETTINGS_FILE_NAME)),
    };

    let settings: MapGenSettings = match from_str(file_string.as_str()) {
        Ok(settings) => settings,
        Err(e) => return Err(format!("Error parsing {}: {}", MAP_SETTINGS_FILE_NAME, e)),
    };

    Ok(settings)
}
