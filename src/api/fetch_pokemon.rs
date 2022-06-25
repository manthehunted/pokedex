use serde::Serialize;

use crate::api::Status;
use std::sync::Arc;

use crate::domain::fetch_pokemon;
use crate::repositories::pokemon::Repository;

#[derive(Serialize)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(number: u16, repo: Arc<dyn Repository>) -> rouille::Response {
    match fetch_pokemon::execute(repo, fetch_pokemon::Request { number }) {
        Ok(fetch_pokemon::Response {
            number,
            name,
            types,
        }) => rouille::Response::json(&Response {
            number,
            name,
            types,
        }),
        Err(fetch_pokemon::Error::NotFound) => Status::NotFound.into(),
        Err(fetch_pokemon::Error::BadRequest) => Status::BadRequest.into(),
        Err(fetch_pokemon::Error::Unknown) => Status::InternalServerError.into(),
    }
}
