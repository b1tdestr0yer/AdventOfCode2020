use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn treeChecker(slope1 : usize, slope2 : usize, n : usize, mat : &Vec<Vec<bool>>) -> usize{

    let mut trees = 0;

    let mut i = 0;
    let mut j = 0;

    while(i < n - 1){
        i = i + slope2;
        j = (j + slope1) % 31;
        if(mat[i][j] == true){
            trees = trees + 1; 
        }
    }
    return trees;
}

fn main() -> Result<(), Error> {
    let path = "input";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut n = 0;
    let mut mat = vec![];

    for line in buffered.lines(){
        let mut aux = vec![];
        for c in line.unwrap().chars(){
            if c == '.' {
                aux.push(false);
            }
            else{
                aux.push(true);
            }
        }
        mat.push(aux);
        n = n + 1;
    }

    let trees = treeChecker(1,1,n,&mat) * treeChecker(3,1,n,&mat) * 
    treeChecker(5,1,n,&mat) * treeChecker(7,1,n,&mat) * treeChecker(1,2,n,&mat);

    println!("{:?}", trees);
    return Ok(());
}