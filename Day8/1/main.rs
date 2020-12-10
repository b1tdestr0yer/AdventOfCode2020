use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut accumulator : i64 = 0;
    let mut v : Vec<(i64, i64, bool)> = vec![];

    for line in buffered.lines() {
        let lineAux = line.unwrap();
        let mut instruction : Vec<&str> = lineAux.split(" ").collect();
        let aux = match instruction[0]{
            "acc" => (0i64, instruction[1].parse::<i64>().unwrap(), false),
            "jmp" => (1i64, instruction[1].parse::<i64>().unwrap(), false),
            "nop" => (2i64, instruction[1].parse::<i64>().unwrap(), false),
            _ => (-1i64, -1i64, true),
        };
        v.push(aux);
    }

    let mut index : i64 = 0;

    while !v[index as usize].2{
        v[index as usize].2 = true;
        if v[index as usize].0 == 0i64{
            accumulator = accumulator + v[index as usize].1;
            index = index + 1;
        }
        else{
            if v[index as usize].0 == 1i64{
                index = index + v[index as usize].1;
            }
            else{
                if v[index as usize].0 == 2i64{
                    index = index + 1;
                }
                else{
                    println!("Panic");
                }
            }
        }
    }

    println!("{:?}", accumulator);
    return Ok(());
}