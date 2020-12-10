use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::cmp;

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut v : Vec<u64> = vec![];
    let result : u64 = 21806024;

    for line in buffered.lines() {
        let lread = line.unwrap();
        let aux : u64 = lread.parse().unwrap();
        v.push(aux);
    }
    let mut sum : u64;
    let mut max : u64;
    let mut min : u64;
    for i in 0..(v.len() - 2){
        sum = v[i] + v[i + 1];
        min = cmp::min(v[i], v[i + 1]);
        max = cmp::max(v[i], v[i + 1]);
        if sum == result{
            println!("{} and {} and their sum is {}", min, max, min+max);
        }
        for j in (i + 2)..v.len(){
            sum = sum + v[j];
            min = cmp::min(min, v[j]);
            max = cmp::max(max, v[j]);
            if sum == result{
                println!("{} and {} and their sum is {}", min, max, min+max);
            }
        }
    }

    return Ok(());
}