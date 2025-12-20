use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match read_file("/etc/hosts") {
        Ok(s) => println!("read {} bytes", s.len()),
        Err(e) => eprintln!("error: {}", e),
    }
}
