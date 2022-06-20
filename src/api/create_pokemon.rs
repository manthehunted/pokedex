use crate::{
    domain::{create_pokemon, entities::Pokemon},
    repositories::pokemon::Repository,
};
use core::fmt;
use std::fmt::Display;
use std::io::Read;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::Status;

#[derive(Deserialize, Serialize)]
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
            Ok(pokemon) => rouille::Response::json(&Response {
                message: serde_json::to_string(&pokemon).expect("expect pokemon response"),
            }),
            Err(create_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
            Err(create_pokemon::Error::Conflict) => rouille::Response::from(Status::Conflict),
            Err(create_pokemon::Error::Unknown) => {
                rouille::Response::from(Status::InternalServerError)
            }
        },
        _ => rouille::Response::from(Status::BadRequest),
    }
}
