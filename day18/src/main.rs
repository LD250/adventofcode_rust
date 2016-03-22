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

    impl Iterator for RectIndexes {
        type Item = usize;
        fn next(&self) -> Option<u32> {
            for r in [-1, 0, 1] {
                for c in [-1, 0, 1] {
                    if ((r != 0) || (c != 0)) && (self.x + r >= 0) && (self.y + c >= 0) && (self.x + r <= 100) && (self.y + c <= 100) {
                        Some(self.r + r, self.y + c)
                    }
                }
            }
            None
        }
    }

    fn get_rect_indedes(middle_index: (usize, usize)) -> [(usize, usize)] {
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
            
            }
        }
    }

    fn new_scheme(scheme: Scheme) -> Scheme {
        let mut new_scheme: Scheme;
        for r in 0..scheme.len() {
            for c in 0..scheme.len() {
                let ids = 
            
            }
        }
    
    }

    println!("{:?}", scheme);

}
