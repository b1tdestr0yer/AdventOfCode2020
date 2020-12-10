use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::collections::HashMap;
use std::iter::FromIterator;

fn birthYear(year : String) -> bool {
    let number : usize = year.parse().unwrap();
    if number >= 1920 && number <= 2002 {
        return true;
    }
    else{
        return false;
    }
}

fn issueYear(year : String) -> bool {
    let number : usize = year.parse().unwrap();
    if number >= 2010 && number <= 2020 {
        return true;
    }
    else{
        return false;
    }
}

fn expirationYear(year : String) -> bool {
    let number : usize = year.parse().unwrap();
    if number >= 2020 && number <= 2030 {
        return true;
    }
    else{
        return false;
    }
}

fn height(hgt : String) -> bool {
    let len : usize = hgt.len();
    let mut v: Vec<char> = hgt.chars().collect();
    if v[len - 2] == 'c'{
        if v[len - 1] == 'm'{
            v.remove(len - 1);
            v.remove(len - 2);
            let number : usize = String::from_iter(v).parse().unwrap();
            if(number >= 150 && number <= 193){
                return true;
            }
            return false;
        }
        else{
            return false;
        }
    }
    else{
        if v[len - 2] == 'i'{
            if v[len - 1] == 'n'{
                v.remove(len - 1);
                v.remove(len - 2);
                let number : usize = String::from_iter(v).parse().unwrap();
                if(number >= 59 && number <= 76){
                    return true;
                }
                return false;
            }
            else{
                return false;
            }
        }
        else{
            return false;
        }
    }
}

fn hairColor(color : String) -> bool {
    let mut v: Vec<char> = color.chars().collect();
    if v[0] == '#' && v.len() == 7{
        for i in 1..6{
            if !v[i].is_alphanumeric() {
                return false;
            }
        }
        return true;
    }
    else{
        return false;
    }
}

fn eyeColor(color : String) -> bool {
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for each in colors{
        if each == color{
            return true;
        }
    }
    return false;
}

fn is_digit(c : char) -> bool {
    let numbs = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for num in numbs{
        if c == num{
            return true;
        }
    }
    return false;
}

fn passportId(id : String) -> bool {
    let mut v: Vec<char> = id.chars().collect();
    
    if v.len() == 9{
        for c in v{
            if !is_digit(c) {
                return false;
            }
        }
        return true;
    }
    else{
        return false;
    }
}

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
            let mut binary : bool = true;
            for (key, val) in dict.iter() {
                
                binary = match key {
                    &"byr" => birthYear(val.to_string()),
                    &"iyr" => issueYear(val.to_string()),
                    &"eyr" => expirationYear(val.to_string()),
                    &"hgt" => height(val.to_string()),
                    &"hcl" => hairColor(val.to_string()),
                    &"ecl" => eyeColor(val.to_string()),
                    &"pid" => passportId(val.to_string()),
                    _ => true,
                };

                if binary == false {
                    break;
                }

            }
            if binary == true {
                valid = valid + 1;
            }
        }
    }

    println!("{:?}", valid);
    return Ok(());
}