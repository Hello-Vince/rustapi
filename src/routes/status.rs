use axum::{routing::get, Json, Router};
use bson::doc;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::errors::Error;

pub fn create_route() -> Router {
  Router::new()
    .route("/status", get(get_status))
    .route("/", get(get_greeting))
}

async fn get_status() -> Result<Json<Status>, Error> {
  debug!("Returning status");
  Ok(Json(Status {
    status: "ok".to_owned(),
  }))
}

async fn get_greeting() -> Result<Json<Status>, Error> {
  debug!("Returning status");
  Ok(Json(Status {
    status: "Welcome to the RustAPI!".to_owned(),
  }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
  status: String,
}
