use std::sync::Arc;

use serde::Serialize;

use crate::{domain::delete_pokemon, repositories::pokemon::Repository};

use super::Status;

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(req: u16, repo: Arc<dyn Repository>) -> rouille::Response {
    match delete_pokemon::execute(req, repo) {
        Ok(delete_pokemon::Response {
            number,
            name,
            types,
        }) => rouille::Response::json(&Response {
            number,
            name,
            types,
        }),
        Err(delete_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(delete_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
        Err(delete_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
    }
}
