#[allow(warnings)]
mod bindings;

use bindings::Guest;

use chrono::Utc;
use crate::bindings::Event;


struct Component;

impl Guest for Component {
    fn get_latest_event_timestamp(event_type: String, user_id: u64) -> String {
        let now = Utc::now();
        format!("The last {event_type} the user {} is at {}", user_id, now.to_rfc3339())
    }

    fn get_player_state(device_type: String) -> String {
        format!("The latest player-state from {} device is buffer", device_type)
    }

    fn get_latest_event_details(device_type: String) -> Event {
        Event {
            event_type: "play".to_string(),
            movie_name: "The Matrix".to_string(),
            device_type,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    fn get_total_play_time(device_type: String) -> Result<u64, String> {
        if device_type == "ios"  {
            Ok(120)
        } else if device_type == "android" {
            Ok(100)
        } else {
            Err("Device type not supported".to_string())
        }
    }

    fn get_total_play_time_of_movie(device_type: String, movie_name: String) -> Result<Option<u64>, String> {
        match device_type.as_str() {
            "ios" => {
                match movie_name.as_str() {
                    "matrix" => Ok(Some(120)),
                    "darknight" => Ok(Some(130)),
                    _ => Ok(None),
                }
            }
            "android" => {
                match movie_name.as_str() {
                    "matrix" => Ok(Some(100)),
                    "darknight" => Ok(Some(110)),
                    _ => Ok(None),
                }
            }
            _ => Err("Device type not supported".to_string())
        }
    }
}

bindings::export!(Component with_types_in bindings);
