use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut validOfOne : u32 = 0;
    let mut validOfTwo : u32 = 0;
    let mut validOfThree : u32 = 0;
    let mut v : Vec<u64> = vec![0];

    for line in buffered.lines() {
        let lineAux = line.unwrap();
        let aux : u64 = lineAux.parse().unwrap();
        v.push(aux);
    }

    v.sort();

    v.push(v[v.len() - 1 ] + 3);

    for i in 0..(v.len() - 1){
        if v[i + 1] - v[i] == 1{
            validOfOne = validOfOne + 1;
        }
        else{
            if v[i + 1] - v[i] == 3{
                validOfThree = validOfThree + 1;
            }
        }
    }

    println!("{} and {}", validOfOne, validOfThree);

    return Ok(());
}