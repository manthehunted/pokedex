use crate::{
    domain::{create_pokemon, entities::Pokemon},
    repositories::pokemon::Repository,
};
use core::fmt;
use std::fmt::Display;
use std::io::Read;
use std::sync::Arc;

use rouille;
use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

impl From<Request> for create_pokemon::Request {
    fn from(req: Request) -> Self {
        Self {
            number: req.number,
            name: req.name,
            types: req.types,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pokemon = name: {}, number: {}, types: {:?}",
            self.number, self.name, self.types
        )
    }
}

#[derive(Serialize)]
struct Response {
    message: String,
}

pub fn serve(req: &rouille::Request, repo: Arc<dyn Repository>) -> rouille::Response {
    match rouille::input::json_input::<Request>(req) {
        Ok(req) => match create_pokemon::execute(repo, req.into()) {
            create_pokemon::Response::Ok(number) => rouille::Response::json(&Response {
                message: format!("successfully inserted {}", number),
            }),
            create_pokemon::Response::BadRequest => rouille::Response::from(Status::BadRequest),
            create_pokemon::Response::Conflict => rouille::Response::from(Status::Conflict),
            create_pokemon::Response::Error => rouille::Response::from(Status::InternalServerError),
        },
        _ => rouille::Response::from(Status::BadRequest),
    }
}
