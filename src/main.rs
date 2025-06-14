mod models;
mod service;
mod repository;

use service::calendar_service::*;
use service::calendar_status::CalendarStatus;
use crate::models::events::Event;

fn main() {
    // Probamos obtener todos los eventos
    match get_all_events() {
        Ok(response) => {
            println!("Todos los eventos:");
            for event in response.events {
                println!(
                    "Título: {}, Fecha: {}, Hora: {}, Descripción: {}",
                    event.title, event.date, event.time, event.description
                );
            }
        }
        Err(error_calendar_status) => println!("Error al obtener eventos: {:?}", error_calendar_status),
    }

    // Probamos buscar un evento por título
    match get_event_by_title("Dentista") {
        Ok(event) => println!("Evento encontrado: {:?}", event),
        Err(CalendarStatus::EventNotFound) => println!("Evento no encontrado"),
        _ => {}
    }
}
