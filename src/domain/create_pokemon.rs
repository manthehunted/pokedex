use super::entities::{PokemonName, PokemonNumber, PokemonTypes};

enum Response {
    Ok(u16),
    BadRequest,
}

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

fn execute(req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(pokemon_number), Ok(_), Ok(_)) => Response::Ok(pokemon_number.into()),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };
        let res = execute(req);

        match res {
            Response::Ok(res) => assert_eq!(res, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_err_when_a_request_is_invalid() {
        let number = 25;
        let req = Request {
            number,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };
        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
