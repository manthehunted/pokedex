use std::sync::Arc;

use super::entities::{PokemonName, PokemonNumber, PokemonTypes};
use crate::repositories::pokemon::{InMemoryRepository, Insert, Repository};

pub enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            Insert::Ok(number) => Response::Ok(number),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let mut repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };
        let res = execute(repo, req);

        match res {
            Response::Ok(res) => assert_eq!(res, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_err_when_a_request_is_invalid() {
        let number = 25;
        let mut repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };
        let res = execute(repo, req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_if_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();

        let mut repo = Arc::new(InMemoryRepository::new());
        repo.insert(number, name, types);
        let req = Request {
            number: 25,
            name: String::from("test"),
            types: vec![String::from("Fire")],
        };

        let res = execute(repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let mut repo = Arc::new(InMemoryRepository::new().with_error());

        let req = Request {
            number: 25,
            name: String::from("test"),
            types: vec![String::from("Fire")],
        };

        let res = execute(repo, req);

        match res {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
