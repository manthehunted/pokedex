use core::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub number: PokemonNumber,
    pub name: PokemonName,
    pub types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
        Self {
            number,
            name,
            types,
        }
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pokemon number: {}, name: {}, types: {}",
            self.number, self.name, self.types
        )
    }
}

#[derive(PartialEq, Clone, Ord, Eq, PartialOrd, Debug)]
pub struct PokemonNumber(u16);

impl Display for PokemonNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "number: {}", self.0)
    }
}

impl TryFrom<u16> for PokemonNumber {
    type Error = ();

    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n > 0 && n < 899 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
}

impl From<PokemonNumber> for u16 {
    fn from(n: PokemonNumber) -> Self {
        n.0
    }
}

#[derive(Clone, Debug)]
pub struct PokemonName(String);

impl Display for PokemonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: {}", self.0)
    }
}

impl TryFrom<String> for PokemonName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<PokemonName> for String {
    fn from(n: PokemonName) -> String {
        n.0
    }
}

#[derive(Clone, Debug)]
pub struct PokemonTypes(Vec<PokemonType>);

impl Display for PokemonTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|t| t.clone().into())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "types: {}", s)
    }
}

impl TryFrom<Vec<String>> for PokemonTypes {
    type Error = ();

    fn try_from(types: Vec<String>) -> Result<Self, Self::Error> {
        if types.is_empty() {
            Err(())
        } else {
            types
                .into_iter()
                .map(PokemonType::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map(PokemonTypes)
        }
    }
}

impl From<PokemonTypes> for Vec<String> {
    fn from(pokemon_types: PokemonTypes) -> Self {
        let mut vs: Vec<String> = Vec::new();
        for pt in pokemon_types.0.into_iter() {
            vs.push(pt.into());
        }
        vs
    }
}

#[derive(Clone, Debug)]
enum PokemonType {
    Electric,
    Fire,
}

impl TryFrom<String> for PokemonType {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Electric" => Ok(Self::Electric),
            "Fire" => Ok(Self::Fire),
            _ => Err(()),
        }
    }
}

impl From<PokemonType> for String {
    fn from(ptype: PokemonType) -> String {
        match ptype {
            PokemonType::Electric => "Electric".to_string(),
            PokemonType::Fire => "Fire".to_string(),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
impl PokemonNumber {
    pub fn pikachu() -> Self {
        Self(25)
    }

    pub fn charmander() -> Self {
        Self(4)
    }

    pub fn bad() -> Self {
        Self(0)
    }
}

#[cfg(test)]
impl PokemonName {
    pub fn pikachu() -> Self {
        Self(String::from("Pikachu"))
    }

    pub fn charmander() -> Self {
        Self(String::from("Charmander"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}

#[cfg(test)]
impl PokemonTypes {
    pub fn pikachu() -> Self {
        Self(vec![PokemonType::Electric])
    }

    pub fn charmander() -> Self {
        Self(vec![PokemonType::Fire])
    }
}
