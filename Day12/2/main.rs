use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn moveLeft(v : &mut Vec<u32>, degress : u32){
    let vCopy = v.clone();
    v[(((360 - degress) % 360) / 90) as usize] = vCopy[0];
    v[(((360 - degress + 90) % 360) / 90) as usize] = vCopy[1];
    v[(((360 - degress + 180) % 360) / 90) as usize] = vCopy[2];
    v[(((360 - degress + 270) % 360) / 90) as usize] = vCopy[3];
}

fn moveRight(v : &mut Vec<u32>, degress : u32){
    let vCopy = v.clone();
    v[((degress % 360) / 90) as usize] = vCopy[0];
    v[(((degress + 90) % 360) / 90) as usize] = vCopy[1];
    v[(((degress + 180) % 360) / 90) as usize] = vCopy[2];
    v[(((degress + 270) % 360) / 90) as usize] = vCopy[3];
}

fn add(mult : u32, v : &mut Vec<u32>, vWay : &Vec<u32>){
    v[0] = v[0] + mult * vWay[0];
    v[1] = v[1] + mult * vWay[1];
    v[2] = v[2] + mult * vWay[2];
    v[3] = v[3] + mult * vWay[3];
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut valid : u32 = 0;
    let mut instructions : Vec<(char, u32)> = vec![];
    let mut v : Vec<u32> = vec![0, 0, 0, 0];
    let mut vWay : Vec<u32> = vec![1, 10, 0, 0];

    for line in buffered.lines() {
        let aux = line.unwrap();
        let a = aux.split_at(1);
        let t : (char, u32) = (a.0.to_string().parse().unwrap(), a.1.parse().unwrap());
        instructions.push(t);
    }

    for i in instructions{
        match i.0 {
            'N' => vWay[0] = vWay[0] + i.1,
            'E' => vWay[1] = vWay[1] + i.1,
            'S' => vWay[2] = vWay[2] + i.1,
            'W' => vWay[3] = vWay[3] + i.1,
            'R' => moveRight(&mut vWay, i.1),
            'L' => moveLeft(&mut vWay, i.1),
            'F' => add(i.1, &mut v, &vWay),
            _ => println!("Bad value!"),
        }
    }

    if v[0] > v[2]{
        valid = valid + v[0] - v[2];
    }
    else{
        valid = valid + v[2] - v[0];
    }

    if v[1] > v[3]{
        valid = valid + v[1] - v[3];
    }
    else{
        valid = valid + v[3] - v[1];
    }

    println!("{:?}", valid);

    return Ok(());
}