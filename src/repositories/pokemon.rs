use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};
use std::sync::Mutex;

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError>;

    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError>;
    fn fetch(&self, number: PokemonNumber) -> Result<Pokemon, FetchError>;
    fn delete(&self, number: PokemonNumber) -> Result<Pokemon, DeleteError>;
}

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum FetchError {
    Unknown,
    NotFound,
}

pub enum DeleteError {
    Unknown,
    NotFound,
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
    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }

        let pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown),
        };
        let mut pokemons = pokemons.to_vec();
        pokemons.sort_by(|pokemon1, pokemon2| pokemon1.number.cmp(&pokemon2.number));
        Ok(pokemons)
    }

    fn fetch(&self, number: PokemonNumber) -> Result<Pokemon, FetchError> {
        if self.error {
            return Err(FetchError::Unknown);
        }

        let pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchError::Unknown),
        };

        let mut iter = pokemons.iter().filter(|pokemon| pokemon.number == number);
        match iter.next() {
            Some(pokemon) => Ok(pokemon.clone()),
            _ => Err(FetchError::NotFound),
        }
    }

    fn delete(&self, number: PokemonNumber) -> Result<Pokemon, DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            _ => return Err(DeleteError::Unknown),
        };

        match pokemons.iter().position(|p| p.number == number) {
            Some(idx) => Ok(pokemons.remove(idx)),
            None => Err(DeleteError::NotFound),
        }
    }
}
