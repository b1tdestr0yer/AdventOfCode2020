use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::convert::TryInto;
use std::cmp;

fn validate(mat : &Vec<Vec<u32>>, i : usize, j : usize) -> bool{
    let iPos : Vec<i32> = vec![-1, -1, -1,  0, 0,  1, 1, 1];
    let jPos : Vec<i32> = vec![-1,  0,  1, -1, 1, -1, 0, 1];

    for c in 1..cmp::max(mat.len(), mat[i].len()){
        for pos in 0..8{
            let iCurrent : i32 = (i as i32 + iPos[pos] * (c as i32) );
            let jCurrent : i32 = (j as i32 + jPos[pos] * (c as i32) );
            if iCurrent >= 0 && iCurrent < mat.len() as i32 && 
            jCurrent >= 0 && jCurrent < mat[i].len() as i32 &&
            mat[iCurrent as usize][jCurrent as usize] == 2{
                return false; 
            }
        }
    }
    return true;
}

fn round(mat : Vec<Vec<u32>>) -> (bool, Vec<Vec<u32>>){
    let mut newMat = mat.clone();
    let iPos : Vec<i32> = vec![-1, -1, -1,  0, 0,  1, 1, 1];
    let jPos : Vec<i32> = vec![-1,  0,  1, -1, 1, -1, 0, 1];
    let mut total : u32 = 0u32;
    let mut changed : bool = false;
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            total = 0u32;
            if mat[i][j] == 1{
                if validate(&mat, i, j){
                    newMat[i][j] = 2;
                    changed = true;
                }
            }
            else{
                if mat[i][j] == 2{
                    for pos in 0..8{
                        let iCurrent : i32 = (i as i32 + iPos[pos]);
                        let jCurrent : i32 = (j as i32 + jPos[pos]);
                        if iCurrent >= 0 && iCurrent < mat.len() as i32 && 
                        jCurrent >= 0 && jCurrent < mat[i].len() as i32 &&
                        mat[iCurrent as usize][jCurrent as usize] == 2 {
                            total = total + 1; 
                        }
                    }
                    if total >= 5{
                        newMat[i][j] = 1;
                        changed = true;
                    } 
                }
            }
        }
    }
    return (changed, newMat);
}

fn main() -> Result<(), Error> {
    let path = "input3";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut mat : Vec<Vec<u32>> = vec![];

    for line in buffered.lines() {
        let aux = line.unwrap();
        let iter = aux.chars();
        let mut v : Vec<u32> = vec![];
        for i in iter{
            let a : u32 = match i {
                'L' => 1u32,
                '.' => 0u32,
                _ => 2u32,
            };
            v.push(a);
        }
        mat.push(v);
    }

    let mut i = 0;
    let mut b : bool = true;
    //println!("{:?}", mat);
    while  b == true{
        let aux = round(mat);
        b = aux.0;
        mat = aux.1;
        i = i + 1;
        //println!("{:?}", mat);
    }

    let mut valid : u32 = 0u32;
    for i in 0..mat.len(){
        for j in 0..mat[i].len(){
            if mat[i][j] == 0{
                print!(".");
            }
            if mat[i][j] == 1{
                print!("L");
            }
            if mat[i][j] == 2{
                print!("#");
                valid = valid + 1;
            }
        }
        print!("\n");
    }

    println!("{:?}", valid);

    return Ok(());
}