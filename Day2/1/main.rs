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
        let minApparition : u32 = v2[0].parse().unwrap();
        let maxApparition : u32 = v2[1].parse().unwrap();
        let c : char = v[1].chars().nth(0).unwrap();
        let mut apparition : u32 = 0;
        for i in v[2].chars() {
            if(i == c){
                apparition = apparition + 1;
            }
        }
        if(!(apparition < minApparition || apparition > maxApparition)){
            valid = valid + 1;
        }
    }

    println!("{}", valid);

    return Ok(());
}