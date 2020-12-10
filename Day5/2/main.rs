use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut list = vec![];

    for line in buffered.lines() {
        let mut v : Vec<char> = line.unwrap().chars().collect();
        let mut startRow = 0;
        let mut stopRow = 127;
        let mut startCol = 0;
        let mut stopCol = 7;
        for i in 0..7{
            if v[i] == 'F' {
                stopRow = (startRow + stopRow) / 2;
            }
            else{
                startRow = (startRow + stopRow + 1) / 2;
            }
        }
        for i in 7..10{
            if v[i] == 'R' {
                startCol = (startCol + stopCol + 1) / 2;
            }
            else{
                stopCol = (startCol + stopCol) / 2;
            }
        }
        let id = startRow * 8 + stopCol;
        list.push(id);
    }

    list.sort();

    for i in 1..list.len()-1{
        if !(list[i] == list[i - 1] + 1 && list[i] == list[i + 1] - 1){
            println!("{}", list[i]);
        }
    }

    return Ok(());
}