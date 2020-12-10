use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut numbers = vec![];

    for line in buffered.lines() {
        let number : u32 = line.unwrap().parse().unwrap();
        numbers.push(number);
    }

    for number in &numbers {
        for number2 in &numbers {
            if number + number2 == 2020 {
                println!("{}", number * number2);
            }
        }
    }

    Ok(())
}