use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn verify(v : Vec<u64>, number : u64) -> bool{
    for each in &v{
        for each2 in &v{
            if each + each2 == number && each != each2{
                return true
            }
        }
    }
    return false;
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut v : Vec<u64> = vec![];

    for line in buffered.lines() {
        let lread = line.unwrap();
        let aux : u64 = lread.parse().unwrap();
        if (v.len() < 25){
            v.push(aux);
        }
        else{
            if( verify(v.clone(), aux) == true){
                v.remove(0);
                v.push(aux);
            }
            else{
                println!("{}", aux);
                return Ok(());
            }
        }
    }

    return Ok(());
}