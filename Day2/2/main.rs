use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut valid : u32 = 0;

    for line in buffered.lines() {
        let aux = line.unwrap();
        let v : Vec<&str> = aux.split(" ").collect();
        let v2 : Vec<&str> = v[0].split("-").collect();
        let okayIndex : usize = v2[0].parse().unwrap();
        let notOkayIndex : usize = v2[1].parse().unwrap();
        let c : char = v[1].chars().nth(0).unwrap();
        let c2 : char = v[2].chars().nth(okayIndex - 1).unwrap();
        let c3 : char = v[2].chars().nth(notOkayIndex - 1).unwrap();
        if((c2 == c && c3 != c) || (c2 != c && c3 == c)){
           valid = valid + 1;
        }
    }

    println!("{}", valid);

    return Ok(());
}