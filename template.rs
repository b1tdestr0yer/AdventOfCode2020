use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut valid : u32 = 0;

    for line in buffered.lines() {
        let aux = line.unwrap();
    }

    println!("{}", valid);

    return Ok(());
}