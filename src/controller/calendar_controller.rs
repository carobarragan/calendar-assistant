use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::sync::Arc;
use async_trait::async_trait;

use crate::service::calendar_service::{CalendarService, CalendarServiceTrait, DynCalendarService};
use crate::service::calendar_status::CalendarStatus;


/// Calendar Controller struct
pub struct CalendarController {
    calendar_service: DynCalendarService,
}

/// Calendar Controller trait
#[async_trait]
pub trait CalendarControllerTrait {
    /// Configure declared endpoints for this controller
     fn config_endpoints(service:DynCalendarService) -> Router;
}
/// Calendar Controller implement logic
#[async_trait]
impl CalendarControllerTrait for CalendarController {
    /// Configure declared endpoints for this controller
    fn config_endpoints(service: Arc<(dyn CalendarServiceTrait + Send + Sync + 'static)>) -> Router{
        Router::new()
            .route("/calendar", axum::routing::get(get_all_events))
            .route("/calendar/:title", axum::routing::get(get_event_by_title))
            .with_state(service)
    }
}

async fn get_all_events(
    State(service): State<DynCalendarService>,
) -> impl IntoResponse {
    match service.get_all_events().await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error getting events").into_response(),
    }
}

async fn get_event_by_title(
    Path(title): Path<String>,
    State(service): State<DynCalendarService>,
) -> impl IntoResponse {
    match service.get_event_by_title(title).await {
        Ok(event) => (StatusCode::OK, Json(event)).into_response(),
        Err(CalendarStatus::EventNotFound) => (StatusCode::NOT_FOUND, "Event not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching event").into_response(),
    }
}