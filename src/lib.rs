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
}

bindings::export!(Component with_types_in bindings);
