use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let path = "input";
    let mut input = File::open(path)?;
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let passports : Vec<&str> = buffer.split("\n\n").collect();
    let mut valid : usize = 0;
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for passport in passports {
        let mut dict = HashMap::new();
        let fields : Vec<&str> = passport.split(" ").collect();
        for field in fields {
            let aux : Vec<&str> = field.split("\n").collect();
            for a in aux {
                let param : Vec<&str> = a.split(":").collect();
                dict.insert(
                    param[0],
                    param[1],
                );
            }
        }
        let mut okay : bool = true;
        for key in &keys {
            if !dict.contains_key(key) {
                okay = false;
            }
                
        }
        if okay == true {
            valid = valid + 1;
        }
    }

    println!("{:?}", valid);
    return Ok(());
}