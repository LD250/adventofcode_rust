extern crate itertools;
use std::io::prelude::*;
use std::fs::File;
use itertools::Itertools;

fn main() {
    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        },
    };

    let containers: Vec<u32> = content.trim().split("\n").map(|c| c.parse().unwrap()).collect();
    let mut comb_count: u32 = 0;
    for c in 4..containers.len() {
        comb_count += containers.iter().combinations_n(c).fold(0, |sum, comb| if comb.iter().fold(0, |sum, x| sum + *x) == 150 {sum + 1} else {sum + 0});
    };
    let mut min_count: u32 = 0;
    for c in 4..containers.len() {
        match containers.iter().combinations_n(c).filter(|comb| comb.iter().fold(0, |sum, x| sum + *x) == 150).count() {
            0 => (),
            c => {
                min_count = c as u32;
                break;
            }
        }

    };
    println!("{}", comb_count);
    println!("{}", min_count);
}
