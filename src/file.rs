// use std::error::Error;
use std::fs::File;
use std::io::Read;

type Contents = String;
type ErrorString = String;

pub fn read(path: &str) -> Result<Contents, ErrorString> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to open `{}`: {}", path, e.to_string())),
    };
    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Ok(_) => Ok(code),
        Err(e) => Err(format!("Unable to read `{}`: {}", path, e.to_string())),
    }
}
