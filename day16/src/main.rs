use std::io::prelude::*;
use std::fs::File;

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


    let aunt_things: Vec<Thing> = content.trim().split("\n").map(|line| {
        let params: Vec<&str> = line.split(" ").collect();
        [(params[2], params[3].parse().unwrap()),
         (params[4], params[5].parse().unwrap()),
         (params[6], params[7].parse().unwrap())] as Thing
    }).collect();

    println!("{:?}", aunt_things);
}
