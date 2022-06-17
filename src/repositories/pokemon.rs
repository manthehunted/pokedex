use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub trait Repository {
    fn insert(&mut self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert;
}

pub enum Insert {
    Ok(u16),
    Conflict,
    Error,
}

pub struct InMemoryRepository {
    pokemons: Vec<Pokemon>,
    error: bool,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let pokemons: Vec<Pokemon> = vec![];
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
    fn insert(&mut self, number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.pokemons.iter().any(|pokemon| pokemon.number == number) {
            Insert::Conflict
        } else {
            self.pokemons.push(Pokemon {
                number: number.clone(),
                name,
                types,
            });

            Insert::Ok(number.into())
        }
    }
}
