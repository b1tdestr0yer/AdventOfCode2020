use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut mat = vec![];
    let mut n = 0;
    let mut trees = 0;

    for line in buffered.lines(){
        let mut aux = vec![];
        for c in line.unwrap().chars(){
            if c == '.' {
                aux.push(0);
            }
            else{
                aux.push(1);
            }
        }
        mat.push(aux);
        n = n + 1;
    }

    let mut i = 0;
    let mut j = 0;

    while(i < n - 1){
        i = i + 1;
        j = (j + 3) % 31;
        if(mat[i][j] == 1){
            trees = trees + 1; 
        }
    }

    println!("{:?}", trees);
    return Ok(());
}