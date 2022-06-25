use crate::repositories::pokemon::{DeleteError, Repository};
use std::sync::Arc;

use super::entities::{Pokemon, PokemonNumber};

pub enum Error {
    Unknown,
    NotFound,
    BadRequest,
}

pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(number: u16, repo: Arc<dyn Repository>) -> Result<Response, Error> {
    match PokemonNumber::try_from(number) {
        Ok(number) => match repo.delete(number) {
            Ok(Pokemon {
                number,
                name,
                types,
            }) => Ok(Response {
                number: number.into(),
                name: name.into(),
                types: Vec::<String>::from(types),
            }),
            Err(DeleteError::Unknown) => Err(Error::Unknown),
            Err(DeleteError::NotFound) => Err(Error::NotFound),
        },
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        domain::entities::{PokemonName, PokemonNumber, PokemonTypes},
        repositories::pokemon::InMemoryRepository,
    };

    #[test]
    fn it_should_return_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = u16::from(PokemonNumber::pikachu());

        match execute(req, repo) {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_badrequest_error_when_a_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = u16::from(PokemonNumber::bad());

        match execute(req, repo) {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_notfound_error_when_a_pokemon_is_not_found() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::charmander(),
            PokemonName::charmander(),
            PokemonTypes::charmander(),
        );
        let req = u16::from(PokemonNumber::pikachu());

        match execute(req, repo) {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_deleted_pokemon_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::charmander(),
            PokemonName::charmander(),
            PokemonTypes::charmander(),
        );
        let req = u16::from(PokemonNumber::charmander());

        match execute(req, repo) {
            Ok(Response) => {}
            _ => unreachable!(),
        }
    }
}
