use std::io::prelude::*;
use std::fs::File;
use std::cmp;

fn main() {
    #[derive(Debug)]
    struct Reindeer<'a> {
        name: &'a str,
        speed: u32,
        fly_time: u32,
        rest_time: u32,
    };
    const TIME_PASSED: u32 = 2503;

    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        },
    };

    let reindeers: Vec<Reindeer> = content.trim().split("\n").map(|line| {
        let reindeer_info: Vec<&str> = line.split(" ").collect();
        Reindeer {name: reindeer_info[0],
                  speed: reindeer_info[3].parse().unwrap(),
                  fly_time: reindeer_info[6].parse().unwrap(),
                  rest_time: reindeer_info[13].parse().unwrap()}
    }).collect();

    let max_distance: u32 = reindeers.iter().map(|r| {
        let cycle_time = r.fly_time + r.rest_time;
        let cycle_count = TIME_PASSED / cycle_time;
        r.speed * r.fly_time * cycle_count + cmp::min(TIME_PASSED - cycle_time * cycle_count, r.fly_time) * r.speed
    }).max().unwrap();

    println!("{:?}", max_distance);
}
