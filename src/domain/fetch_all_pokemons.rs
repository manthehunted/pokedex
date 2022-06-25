use std::sync::Arc;

use crate::repositories::pokemon::{FetchAllError, Repository};

pub enum Error {
    Unknown,
}

pub struct Response {
    pub name: String,
    pub number: u16,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>) -> Result<Vec<Response>, Error> {
    let o = repo.fetch_all();
    match o {
        Ok(pokemons) => Ok(pokemons
            .into_iter()
            .map(|pokemon| Response {
                name: pokemon.name.into(),
                number: pokemon.number.into(),
                types: Vec::<String>::from(pokemon.types),
            })
            .collect()),
        Err(FetchAllError::Unknown) => Err(Error::Unknown),
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
        let res = execute(repo);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_all_pokemons_ordered_by_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());

        repo.insert(
            PokemonNumber::pikachu(),
            PokemonName::pikachu(),
            PokemonTypes::pikachu(),
        );
        repo.insert(
            PokemonNumber::charmander(),
            PokemonName::charmander(),
            PokemonTypes::charmander(),
        );
        let res = execute(repo);

        match res {
            Ok(res) => {
                assert_eq!(res[0].number, u16::from(PokemonNumber::charmander()));
                assert_eq!(res[0].name, String::from(PokemonName::charmander()));
                assert_eq!(
                    res[0].types,
                    Vec::<String>::from(PokemonTypes::charmander())
                );
                assert_eq!(res[1].number, u16::from(PokemonNumber::pikachu()));
                assert_eq!(res[1].name, String::from(PokemonName::pikachu()));
                assert_eq!(res[1].types, Vec::<String>::from(PokemonTypes::pikachu()));
            }
            _ => unreachable!(),
        }
    }
}
