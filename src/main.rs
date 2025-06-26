mod repository;
mod models;
mod controller;
mod service;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;

use crate::controller::calendar_controller::{CalendarController, CalendarControllerTrait};
use crate::repository::calendar_repository::{CalendarRepository, DynCalendarRepository};
use crate::service::calendar_service::{CalendarService, DynCalendarService};

#[tokio::main]
async fn main() {
    let repository: DynCalendarRepository = Arc::new(CalendarRepository {});
    let service: DynCalendarService = Arc::new(CalendarService {
        calendar_repository: CalendarRepository {},
        repository: repository.clone(),
    });

    let app = CalendarController::config_endpoints(service);

    // 🚀 Levantamos el servidor en localhost:3000
    println!("🚀 Listening on http://localhost:3000");
    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    // Usamos axum::Server en lugar de hyper::Server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}