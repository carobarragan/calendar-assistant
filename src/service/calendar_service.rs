use super::calendar_status::CalendarStatus;

use crate::models::event_response::EventResponse;
use crate::models::events::Event;

pub fn get_all_events() -> Result<EventResponse, CalendarStatus> {
    let events = vec![
        Event {
            title: "Reunión".to_string(),
            date: "2025-06-05".to_string(),
            time: "10:00".to_string(),
            description: "Reunión semanal".to_string(),
        },
        Event {
            title: "Dentista".to_string(),
            date: "2025-06-06".to_string(),
            time: "15:00".to_string(),
            description: "Control anual".to_string(),
        },
    ];

    Ok(EventResponse { events })
}

pub fn get_event_by_title(title: &str) -> Result<Event, CalendarStatus> {
    let response = get_all_events()?;
    response
        .events
        .into_iter()
        .find(|e| e.title == title)
        .ok_or(CalendarStatus::EventNotFound)
}
