#[allow(warnings)]
mod bindings;

use std::cell::RefCell;
use bindings::Guest;

use chrono::Utc;
use crate::bindings::{Change, Connection, Event, EventDetails, EventType, InsertParams};


struct Component;

struct EventAccumulator {
    events: Vec<Event>
}

thread_local! {
    static EVENT_ACCUMULATOR: RefCell<EventAccumulator> = RefCell::new(EventAccumulator {
        events: vec![],
    });
}

fn with_new_event<T>(
    f: impl FnOnce(&mut EventAccumulator) -> Result<T, String>,
) -> Result<T, String> {
    EVENT_ACCUMULATOR.with_borrow_mut(|state| f(state))
}


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
        if device_type == "ios" {
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

    fn add_event(event: Event) -> Result<String, String> {
        with_new_event(|state| {
            if event.device_type == "ios" || event.device_type == "android" {
                state.events.push(event);
                Ok("Event added successfully".to_string())
            } else {
                Err("Invalid event. Device type not supported".to_string())
            }
        })
    }

    fn get_latest_time_of(event_type: String) -> EventDetails {
        EventDetails {
            event_type: EventType::Buffer,
            timestamp: Utc::now().to_rfc3339(),
            movie_name: "matrix".to_string(),
            device_type: "ios".to_string(),
        }
    }

    fn unit_function() -> String {
        "This function doesn't do anything".to_string()
    }

    fn poll(c: Connection) -> Vec<Change> {
        let added = Change::Added("added".to_string());
        let removed = Change::Deleted("removed".to_string());
        let inserted = Change::Inserted(InsertParams {
            after: "inserted".to_string(),
            value: "hello".to_string(),
        });
        vec![added, removed, inserted]
    }

    fn add(c: Connection, value: String) {
        dbg!(c, value);
    }

    fn noop() {
        dbg!("noop");
    }
}

bindings::export!(Component with_types_in bindings);
