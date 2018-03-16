#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_json;

use serde_json::Value;

#[derive(Fail, Debug)]
#[fail(display = "Several errors occured: {:?}", errors)]
pub struct CompoundError {
    errors: Vec<JsonError>,
}

#[derive(Fail, Debug)]
#[fail(display = "JSON was missing field: {}", missing_field)]
pub struct JsonError {
    missing_field: String,
}

type Validated<T> = Result<T, CompoundError>;

fn check_json(json: Value) -> Validated<Value> {
    let mut errors = vec![];

    if let None = json.get("author") {
        errors.push(JsonError {
            missing_field: "author".to_string(),
        })
    }

    if let None = json.get("email") {
        errors.push(JsonError {
            missing_field: "email".to_string(),
        })
    }

    if let None = json.get("git") {
        errors.push(JsonError {
            missing_field: "git".to_string(),
        })
    }
    if errors.is_empty() {
        Ok(json)
    } else {
        Err(CompoundError { errors: errors })
    }
}

fn main() {
    let json = json!({
        "A": ["a", "á", "à"],
        "B": ["b", "b́"],
        "C": ["c", "ć", "ć̣", "ḉ"],
    });
    match check_json(json) {
        Ok(_) => println!("We're good!"),
        Err(e) => for error in e.errors {
            println!("{}", error);
        },
    }
}
