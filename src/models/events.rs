use serde::{Deserialize, Serialize};

/// pub struct event
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
  pub  title: String,
  pub  date: String,
  pub  time: String,
  pub  description: String,
}



