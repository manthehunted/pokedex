use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};
use std::sync::Mutex;

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError>;
}

pub enum InsertError {
    Conflict,
    Unknown,
}

pub struct InMemoryRepository {
    pokemons: Mutex<Vec<Pokemon>>,
    error: bool,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let pokemons: Mutex<Vec<Pokemon>> = Mutex::new(vec![]);
        Self {
            pokemons,
            error: false,
        }
    }

    pub fn with_error(mut self) -> Self {
        self.error = true;
        self
    }
}
impl Repository for InMemoryRepository {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        if pokemons.iter().any(|pokemon| pokemon.number == number) {
            Err(InsertError::Conflict)
        } else {
            let pokemon = Pokemon {
                number,
                name,
                types,
            };
            pokemons.push(pokemon.clone());

            Ok(pokemon)
        }
    }
}
