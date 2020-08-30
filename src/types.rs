use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy)]
pub enum Status {
  todo,
  inProgress,
  done,
}

impl Status {
  pub fn get_statuses() -> Vec<String> {
    vec![Self::to_string(Status::done), Self::to_string(Status::todo), Self::to_string(Status::inProgress)]
  }

  pub fn to_string(status: Self) -> String {
    match status {
      Status::todo => "todo".to_string(),
      Status::inProgress => "inProgress".to_string(),
      Status::done => "done".to_string()
    }
  }
}

impl std::fmt::Debug for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     match *self {
       Status::todo => write!(f, "todo"),
       Status::inProgress => write!(f, "inProgress"),
       Status::done => write!(f, "done")
     }
  } 
}

impl std::fmt::Display for Status {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     match *self {
       Status::todo => write!(f, "todo"),
       Status::inProgress => write!(f, "in_progress"),
       Status::done => write!(f, "done")
     }
  } 
}

impl std::clone::Clone for Status {
  fn clone(&self) -> Self {
    *self
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub status: Status,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Todo {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub items: Vec<Task>,
}