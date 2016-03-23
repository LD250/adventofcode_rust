use std::io::prelude::*;
use std::fs::File;

fn main() {

    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        },
    };
    type Scheme = Vec<Vec<bool>>;
    
    let scheme: Scheme = content.trim().split("\n").map(|line| {
        line.chars().map(|c| if c == '.' {false} else {true}).collect()
    }).collect();

    /*
    fn get_rect_indedes(middle_index: (usize, usize)) -> [(usize, usize)] {
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
            
            }
        }
    }*/
    // let coords: Vec<(i8, i8)> = (-1..2).map(|x: i8| (-1..2).map(|y: i8| (x, y))).iter().flat_map(|c: (i8, i8)| c).collect();
    let coords: Vec<(i8, i8)> = (-1..2).map(|x| (-1..2).map(|y| (x.clone(), y.clone())).collect()).flat_map(|c: Vec<(i8, i8)>| c).collect();


    fn new_scheme(scheme: Scheme) -> Scheme {
        let mut new_scheme: Scheme = vec![];
        for r in 0..scheme.len() {
            for c in 0..scheme.len() {
                let on_count = 1;//RectIndexes {x: r, y: c}.filter(|coord| scheme[coord.0][coord.1] == true).count();
                match scheme[r][c] {
                    true if on_count == 2 || on_count == 3 => new_scheme[r][c] = true,
                    true => new_scheme[r][c] = false,
                    false if on_count ==3 => new_scheme[r][c] = true,
                    false => new_scheme[r][c] = false,
                }
            
            }
        }
        new_scheme
    
    }

    println!("{:?}", coords);

}
