use std::sync::Arc;
use async_trait::async_trait;
use log::{debug, error, info};
use super::calendar_status::CalendarStatus;
use crate::repository::calendar_repository::{CalendarRepository, CalendarRepositoryTrait};
use crate::models::event_response::EventResponse;
use crate::models::events::Event;
use crate::service::calendar_status::CalendarStatus::EventNotFound;

/// Calendar Service struct
pub struct CalendarService {
    calendar_repository: CalendarRepository
}

/// Calendar Service trait
#[async_trait]
pub trait CalendarServiceTrait {
    /// Gets all events in repository
    /// return [EventResponse] on success or [CalendarStatus] on failure
 async fn get_all_events(&self) -> Result<EventResponse, CalendarStatus>;
    /// Gets event by title in repository
    /// on [String] title
    /// return [Event] on success or [CalendarStatus] on failure
   async fn get_event_by_title(&self,title: String) -> Result<Event, CalendarStatus>;

}
/// Calendar Service implemetation logic
#[async_trait]
impl CalendarServiceTrait for CalendarService {
    /// Gets all events in repository
    /// return [EventResponse] on success or [CalendarStatus] on failure
    async fn get_all_events(&self) -> Result<EventResponse, CalendarStatus> {
      match self.calendar_repository.fetch_all_events().await{
          Ok(events)=>{
              info!(" get_all_events - done {:?}", events);
              Ok(EventResponse { events })

          }
          Err(error)=>{
              info!("get_all_events - error  {:?}", error);
              Err(CalendarStatus::InternalError(error.to_string()))
          }
      }

    }

    /// Gets event by title in repository
    /// on [String] title
    /// return [Event] on success or [CalendarStatus] on failure
    async fn get_event_by_title(&self,title: String) -> Result<Event, CalendarStatus> {
        let events = self.calendar_repository.fetch_all_events().await?;
        events.into_iter().find(|event| event.title == title)
            .ok_or(EventNotFound)
    }
}

/// Calendar service trait dyn type
pub type DynCalendarService = Arc<dyn CalendarServiceTrait + Send + Sync>;





