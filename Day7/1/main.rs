use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};
use std::collections::HashMap;
use std::iter::FromIterator;

struct Sacosa{
    name: String,
    number: u64,
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    //let mut valid : u64 = 0;
    let mut hashBags = HashMap::new();
    let mut goodBags = vec![];

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

    let mut verifiedBags = vec![];
    goodBags.push("shiny gold".to_string());
    // verfiedBags.push("shiny gold".to_string());
    // for (bagFirst, bagSecond) in &hashBags{
    //     for i in bagSecond{
    //         if i.name == "shiny gold"{
    //             goodBags.push(bagFirst.to_string());
    //         }
    //     }
    // }
    
    while goodBags.len() != 0{
        for (bagFirst, bagSecond) in &hashBags{
            if !verifiedBags.contains(bagFirst){
                for i in bagSecond{
                    if i.name == goodBags[0]{
                        goodBags.push(bagFirst.to_string());
                    }
                }
            }
        }
        verifiedBags.push(goodBags[0].clone());
        goodBags.remove(0);
    }

    verifiedBags.sort();
    verifiedBags.dedup();

    println!("{:?}", verifiedBags.len() - 1);
    return Ok(());
}