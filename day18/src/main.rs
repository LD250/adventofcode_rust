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

    struct RectIndexes {
        x: usize,
        y: usize,
    }

    /*
    impl Iterator for RectIndexes {
        type Item = (usize, usize);
        fn next(&mut self) -> Option<(usize, usize)> {
            for r in [-1, 0, 1].iter() {
                for c in [-1, 0, 1] {
                    if ((r != 0) || (c != 0)) && (self.x + r >= 0) && (self.y + c >= 0) && (self.x + r <= 100) && (self.y + c <= 100) {
                        Some((self.r + r, self.y + c))
                    }
                }
            }
            None
        }
    }
    */

/*    fn get_rect_indedes(middle_index: (usize, usize)) -> [(usize, usize)] {
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
            
            }
        }
    }*/
    // let coords: Vec<(i8, i8)> = (-1..2).map(|x: i8| (-1..2).map(|y: i8| (x, y))).iter().flat_map(|c: (i8, i8)| c).collect();
    fn neighbours_on(scheme: &Scheme, r: i8, c: i8) -> i8 {
        (-1..2).map(|x| (-1..2).map(|y| (x.clone(), y.clone())).collect())
                               .flat_map(|co: Vec<(i8, i8)>| co)
                               .filter(|&(x, y)| ((x != 0) || (y != 0)) && (r + x >= 0) && (r + x < 100) && (c + y >= 0) && (c + y < 100))
                               .map(|(x, y)| ((x + r) as usize, (y + c) as usize))
                               //.inspect(|x| println!("!! {:?}", x))
                               .filter(|&(x, y)| scheme[x][y] == true)
                               .count() as i8
    }

    fn new_scheme(scheme: &Scheme) -> Scheme {
        let mut new_scheme: Scheme = vec![];
        for r in 0..scheme.len() {
            new_scheme.push(vec![]);
            for c in 0..scheme.len() {
                let on_count = neighbours_on(&scheme, r as i8, c as i8);
                match scheme[r][c] {
                    true if on_count == 2 || on_count == 3 => new_scheme[r].push(true),
                    true => new_scheme[r].push(false),
                    false if on_count == 3 => new_scheme[r].push(true),
                    false => new_scheme[r].push(false),
                }
                // println!("{:?}", new_scheme);
            
            }
        }
        new_scheme
    
    }
    println!("{:?}", scheme[0]);
    let mut i = 1;
    loop {
        let scheme = new_scheme(&scheme);
        i += 1;
        if i > 100 {
            let cc = scheme.iter().map(|line: Vec<bool>| line.iter().filter(|&b| *b).count());//.fold(0, |sum, c| sum + c);
            println!("{:?}", cc); 
            break;
        }
    }

}
