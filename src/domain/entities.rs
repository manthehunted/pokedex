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

#[derive(PartialEq, Clone)]
pub struct PokemonNumber(u16);

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
    fn from(n: PokemonNumber) -> u16 {
        n.0
    }
}

pub struct PokemonName(String);

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

pub struct PokemonTypes(Vec<PokemonType>);

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
