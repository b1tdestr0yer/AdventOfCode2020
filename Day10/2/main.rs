use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut validOfOne : u32 = 0;
    let mut validOfTwo : u32 = 0;
    let mut validOfThree : u32 = 0;
    let mut v : Vec<(u64, u64)> = vec![(0,1)];

    for line in buffered.lines() {
        let lineAux = line.unwrap();
        let jolt : u64 = lineAux.parse().unwrap();
        v.push((jolt, 0u64));
    }

    v.sort_by_key(|k| k.0);
    v.push(((v[v.len() - 1 ].0 + 3u64), 0u64));

    for i in 0..(v.len() - 1){
        v[i + 1].1 = v[i + 1].1 + v[i].1;
        if (i + 2) < v.len() && (v[i + 2].0 - v[i].0) <= 3{
            v[i + 2].1 = v[i + 2].1 + v[i].1;
        }
        if (i + 3) < v.len() && (v[i + 3].0 - v[i].0) <= 3{
            v[i + 3].1 = v[i + 3].1 + v[i].1;
        } 
    }

    println!("{:?}", v[v.len() - 1].1);

    return Ok(());
}