use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn run(mut v : Vec<(i64, i64, bool)>) -> (bool, i64) {
    let mut index : i64 = 0;
    let mut accumulator : i64 = 0;
    
    while index as usize != v.len() && !v[index as usize].2{
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
                    return (false, accumulator);
                }
            }
        }
    }
    if index as usize == v.len(){
        return (true, accumulator);
    }
    else{
        return (false, accumulator);
    }
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    
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

    let mut index = 0usize;
    while index != v.len() as usize{
        if v[index].0 == 1i64{
            v[index].0 = 2i64;
            let aux = run(v.clone());
            if aux.0 == true{
                println!("{:?}", aux.1);
                return Ok(());
            }
            v[index].0 = 1i64;
        }
        else{
            if v[index].0 == 2i64{
                v[index].0 = 1i64;
                let aux = run(v.clone());
                if aux.0 == true{
                    println!("{:?}", aux.1);
                    return Ok(());
                }
                v[index].0 = 2i64;
            }
        }
        index = index + 1;
    }

    //println!("{:?}", v);
    return Ok(());
}