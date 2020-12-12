use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut valid : u32 = 0;
    let mut instructions : Vec<(char, u32)> = vec![];
    let mut degress : u32 = 90;
    let mut v : Vec<u32> = vec![0, 0, 0, 0];

    for line in buffered.lines() {
        let aux = line.unwrap();
        let a = aux.split_at(1);
        let t : (char, u32) = (a.0.to_string().parse().unwrap(), a.1.parse().unwrap());
        instructions.push(t);
    }

    for i in instructions{
        match i.0 {
            'N' => v[0] = v[0] + i.1,
            'E' => v[1] = v[1] + i.1,
            'S' => v[2] = v[2] + i.1,
            'W' => v[3] = v[3] + i.1,
            'R' => degress = (degress + i.1) % 360,
            'L' => degress = (360 + degress - i.1) % 360,
            'F' => v[(degress / 90) as usize] = v[(degress / 90) as usize] + i.1,
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