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
        x: i8,
        y: i8,
        modx: i8,
        mody: i8,
    }

    impl Iterator for RectIndexes {
        type Item = (usize, usize);
        fn next(&mut self) -> Option<(usize, usize)> {
            for r in [-1, 0, 1].iter() {
                for c in [-1, 0, 1].iter() {
                    if ((*r != 0) || (*c != 0)) && (self.x + *r >= 0) && (self.y + *c >= 0) && (self.x + *r <= 100) && (self.y + *c <= 100) {
                        return Some(((self.x + r) as usize, (self.y + c) as usize));
                    }
                }
            }
            None
        }
    }

/*    fn get_rect_indedes(middle_index: (usize, usize)) -> [(usize, usize)] {
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
            
            }
        }
    }*/

    fn new_scheme(scheme: Scheme) -> Scheme {
        let mut new_scheme: Scheme = vec![];
        for r in 0..scheme.len() {
            for c in 0..scheme.len() {
                println!("{:?}", c);
                let mut i = RectIndexes {x: r as i8, y: c as i8};
                println!("{:?}", i.next());
                println!("{:?}", i.next());
                println!("{:?}", i.next());
                println!("{:?}", i.next());
                let on_count = RectIndexes {x: r as i8, y: c as i8}.filter(|coord| scheme[coord.0][coord.1] == true).count();
                match scheme[r][c] {
                    true if on_count == 2 || on_count == 3 => new_scheme[r][c] = true,
                    true => new_scheme[r][c] = false,
                    false if on_count ==3 => new_scheme[r][c] = true,
                    false => new_scheme[r][c] = false,
                }
                // println!("{:?}", new_scheme);
            
            }
        }
        new_scheme
    
    };

    println!("{:?}", new_scheme(scheme));

}
