use std::{
    env::{self},
    fs::File,
    io::{self, Error, ErrorKind},
};

fn main() -> Result<(), io::Error> {
    let path = env::args().nth(1);
    if path.is_none() {
        println!("Usage:  runit <script>");
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    let path = path.expect("wtf, error handled -0x69");

    let script_file = io::read_to_string(File::open(path)?)?;
    println!("{}", script_file);

    return Ok(());
}
