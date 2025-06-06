#[derive(Debug)]
pub enum CalendarStatus{
    EventNotFound,
    InternalError(String),
}
