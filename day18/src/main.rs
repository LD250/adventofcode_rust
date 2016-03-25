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

    fn neighbours_on(scheme: &Scheme, r: i8, c: i8) -> i8 {
        (-1..2).map(|x| (-1..2).map(|y| (x.clone(), y.clone())).collect())
                               .flat_map(|co: Vec<(i8, i8)>| co)
                               .filter(|&(x, y)| ((x != 0) || (y != 0)) && (r + x >= 0) && (r + x < 100) && (c + y >= 0) && (c + y < 100))
                               .map(|(x, y)| ((x + r) as usize, (y + c) as usize))
                               //.inspect(|x| println!("!! {:?}", x))
                               .filter(|&(x, y)| scheme[x][y] == true)
                               .count() as i8
    }


    fn get_new_scheme(scheme: &Scheme, stucked_corners: bool) -> Scheme {
        let mut new_scheme: Scheme = vec![];
        for r in 0..scheme.len() {
            new_scheme.push(vec![]);
            for c in 0..scheme.len() {
                if stucked_corners && ((r == 0 && c == 0) || (r == 0 && c == 99) || (r == 99 && c == 0) || (r == 99 && c == 99)) {
                    new_scheme[r].push(true);
                } else {
                    let on_count = neighbours_on(&scheme, r as i8, c as i8);
                    match scheme[r][c] {
                        true if on_count == 2 || on_count == 3 => new_scheme[r].push(true),
                        true => new_scheme[r].push(false),
                        false if on_count == 3 => new_scheme[r].push(true),
                        false => new_scheme[r].push(false),
                    }
                }
            
            }
        }
        new_scheme
    
    }

    fn count_on(scheme: Scheme) -> usize {
        scheme.iter().map(|l| l.iter().filter(|&x| *x==true).count()).fold(0, |sum, x| sum + x)
    }

    fn light_on_after_steps(scheme: &Scheme, steps_count: usize, stucked_corners: bool) -> usize {
        let mut i = 1;
        let mut scheme1 = scheme.clone();
        loop {
            scheme1 = get_new_scheme(&scheme1, stucked_corners);
            i += 1;
            if i > steps_count {
                return count_on(scheme1);
            }
        }
    }

    println!("{:?}", light_on_after_steps(&scheme, 100, false)); 
    println!("{:?}", light_on_after_steps(&scheme, 100, true)); 

}
