use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};

fn main() -> Result<(), Error> {
    let mut alphabet : Vec<String> = vec![];
    let path = "input";
    let mut input = File::open(path)?;
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let groups : Vec<&str> = buffer.split("\n\n").collect();
    let mut valid = 0;

    for i in 0..26{
        alphabet.push(String::from(((97 + i) as u8) as char));
    }

    for group in groups{
        for alph in &alphabet{
            if group.contains(alph){
                valid = valid + 1;
            }
        }
    }

    println!("{}", valid);
    return Ok(());
}