use std::sync::Arc;

use crate::repositories::pokemon::{FetchError, Repository};

use super::entities::PokemonNumber;

pub enum Error {
    Unknown,
    BadRequest,
    NotFound,
}

pub struct Request {
    pub number: u16,
}

pub struct Response {
    pub name: String,
    pub number: u16,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    let pokemon_number = match PokemonNumber::try_from(req.number) {
        Ok(pokemon_number) => pokemon_number,
        _ => return Err(Error::BadRequest),
    };

    let res = repo.fetch(pokemon_number);
    match res {
        Ok(pokemon) => Ok(Response {
            name: pokemon.name.into(),
            number: pokemon.number.into(),
            types: Vec::<String>::from(pokemon.types),
        }),
        Err(FetchError::Unknown) => Err(Error::Unknown),
        Err(FetchError::NotFound) => Err(Error::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{PokemonName, PokemonNumber, PokemonTypes};
    use crate::repositories::pokemon::InMemoryRepository;

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryRepository::new().with_error());
        let req = Request {
            number: PokemonNumber::pikachu().into(),
        };
        let res = execute(repo, req);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number: PokemonNumber::bad().into(),
        };
        let res = execute(repo, req);

        match res {
            Err(Error::BadRequest) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_not_found_error_when_repo_doesnot_contain_the_pokemon() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok()
        .expect("pokemon to be inserted");
        let req = Request {
            number: PokemonNumber::charmander().into(),
        };
        let res = execute(repo, req);

        match res {
            Err(Error::NotFound) => {}
            _ => unreachable!(),
        }
    }
    #[test]
    fn it_should_return_a_pokemon_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        )
        .ok()
        .expect("pokemon to be inserted");
        let req = Request {
            number: PokemonNumber::pikachu().into(),
        };
        let res = execute(repo, req);

        match res {
            Ok(Response {
                number,
                name,
                types,
            }) => {
                assert_eq!(number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(name, String::from(PokemonName::pikachu()));
                assert_eq!(types, Vec::<String>::from(PokemonTypes::pikachu()));
            }
            _ => unreachable!(),
        }
    }
}
