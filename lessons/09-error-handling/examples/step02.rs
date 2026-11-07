use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;   // on Err: return it now
    Ok(text.trim().to_uppercase())
}

fn main() {
    fs::write("app.conf", "  debug=true  \n").unwrap();
    match read_config("app.conf") {
        Ok(c) => println!("config: {c}"),
        Err(e) => println!("error: {e}"),
    }
    match read_config("missing.conf") {
        Ok(c) => println!("config: {c}"),
        Err(e) => println!("error: {e}"),
    }
}
