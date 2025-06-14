use std::sync::Arc;
use async_trait::async_trait;
use log::info;
use crate::models::event_response::EventResponse;
use crate::models::events::Event;
use crate::service::calendar_status::CalendarStatus;

/// Calendar Repository struct
pub struct CalendarRepository {}

/// Calendar Repository trait
#[async_trait]
pub trait CalendarRepositoryTrait {
    /// Fetch all events in repository
    /// return [Vec<Event>] on success or [CalendarStatus] on failure
    async fn fetch_all_events(&self) -> Result<Vec<Event>, CalendarStatus>;
}
/// Calendar Repository implement logic
#[async_trait]
impl CalendarRepositoryTrait for CalendarRepository {
    /// Fetch all events in repository
    /// return [Vec<Event>] on success or [CalendarStatus] on failure
    async fn fetch_all_events(&self) -> Result<Vec<Event>, CalendarStatus>{
        info!("fetch_all_events - done");
        Ok(vec![
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
        ])
    }

}

/// Calendar repository trait dyn type
pub type DynCalendarRepository = Arc<dyn CalendarRepositoryTrait + Send + Sync>;

