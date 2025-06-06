use super::events::Event;
use serde::{Deserialize, Serialize};

/// pub struct event response
#[derive(Serialize, Deserialize, Debug)]
pub struct EventResponse {
    pub events: Vec<Event>,
}
