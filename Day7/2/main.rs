use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::collections::HashMap;
use std::iter::FromIterator;

struct Sacosa{
    name: String,
    number: u64,
}

fn recurs(hash : &HashMap<String, Vec<Sacosa>>, elem : String) -> u64{
    let mut valid : u64 = 0;
    for (firstBag, vectorBag) in hash{
        if firstBag.to_string() == elem{
            for i in vectorBag{
                valid = valid + i.number * recurs(&hash, i.name.clone()) + i.number;
            }
            return valid;
        }
    }
    return 0;
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut valid : u64 = 0;
    let mut hashBags = HashMap::new();

    for line in buffered.lines() {
        let mut lineRead = line.unwrap();
        let mut aux : Vec<&str> = lineRead.split(" contain ").collect();
        let mut bagStore = String::from(aux[0].get(0..(aux[0].len() - 5)).unwrap());
        let mut bagToStore : Vec<Sacosa> = vec![];
        let mut bags : Vec<&str> = aux[1].split(", ").collect();
        if bags.contains(&"no other bags."){
            continue;
        }
        else{
            if bags.len() != 1 {
                for i in 0..(bags.len() - 1){
                    let num : u64 = String::from(bags[i].get(0..1).unwrap()).parse().unwrap();
                    if num == 1{
                        let xua = Sacosa {
                            name: String::from(bags[i].get(2..(bags[i].len() - 4)).unwrap()),
                            number: num,
                        };
                        bagToStore.push(xua);
                    }
                    else{
                        let xua = Sacosa {
                            name: String::from(bags[i].get(2..(bags[i].len() - 5)).unwrap()),
                            number: num,
                        };
                        bagToStore.push(xua);
                    }
                }
                let num : u64 = String::from(bags[bags.len() - 1].get(0..1).unwrap()).parse().unwrap();
                if num == 1{
                    let xua = Sacosa {
                        name: String::from(bags[bags.len() - 1].get(2..(bags[bags.len() - 1].len() - 5)).unwrap()),
                        number: num,
                    };
                    bagToStore.push(xua);
                }
                else{
                    let xua = Sacosa {
                        name: String::from(bags[bags.len() - 1].get(2..(bags[bags.len() - 1].len() - 6)).unwrap()),
                        number: num,
                    };
                    bagToStore.push(xua);
                }
            }
            else{
                let num : u64 = String::from(bags[0].get(0..1).unwrap()).parse().unwrap();
                if num == 1{
                    let xua = Sacosa {
                        name: String::from(bags[0].get(2..(bags[0].len() - 5)).unwrap()),
                        number: num,
                    };
                    bagToStore.push(xua);
                }
                else{
                    let xua = Sacosa {
                        name: String::from(bags[0].get(2..(bags[0].len() - 6)).unwrap()),
                        number: num,
                    };
                    bagToStore.push(xua);
                }
            }
        }
        hashBags.insert(
            bagStore,
            bagToStore,
        );
    }
    
    valid = recurs(&hashBags, "shiny gold".to_string());

    println!("{:?}", valid);
    return Ok(());
}