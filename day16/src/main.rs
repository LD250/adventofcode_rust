use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
            content = content.replace(",", "").replace(":", "");
        },
    };

    type Thing<'a> = [(&'a str, u8); 3];

    let aunts_things: Vec<Thing> = content.trim().split("\n").map(|line| {
        let params: Vec<&str> = line.split(" ").collect();
        [(params[2], params[3].parse().unwrap()),
         (params[4], params[5].parse().unwrap()),
         (params[6], params[7].parse().unwrap())] as Thing
    }).collect();

    let mut content = String::new();

    match File::open("data/ticker_tape.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
            content = content.replace(":", "");
        },
    };


    let mut known_things: HashMap<&str, u8> = HashMap::new();
    content.trim().split("\n").map(|line| {
        let params: Vec<&str> = line.split(" ").collect();
        known_things.insert(params[0], params[1].parse().unwrap())
    });

    
    println!("{:?}", known_things);
}
