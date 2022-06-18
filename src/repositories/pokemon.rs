use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};
use std::sync::Mutex;

pub trait Repository: Send + Sync {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert;
}

pub enum Insert {
    Ok(u16),
    Conflict,
    Error,
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
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Insert::Error,
        };

        if pokemons.iter().any(|pokemon| pokemon.number == number) {
            Insert::Conflict
        } else {
            pokemons.push(Pokemon {
                number: number.clone(),
                name,
                types,
            });

            Insert::Ok(number.into())
        }
    }
}
