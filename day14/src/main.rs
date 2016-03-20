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

    fn cur_position(r: &Reindeer, cur_second: &u32) -> u32 {
        let cycle_time = r.fly_time + r.rest_time;
        let cycle_count = cur_second / cycle_time;
        r.speed * r.fly_time * cycle_count + cmp::min(cur_second - cycle_time * cycle_count, r.fly_time) * r.speed
    }

    let max_distance: u32 = reindeers.iter().map(|r| cur_position(r, &TIME_PASSED)).max().unwrap();
    let reindeers_count = reindeers.len();
    let mut scores: Vec<u32> = vec![0; reindeers_count];
    let mut cur_second: u32 = 1;
    
    while cur_second <= TIME_PASSED {
        let r_distances: Vec<(usize, u32)> = (0..reindeers_count).map(|r_id| (r_id, cur_position(&reindeers[r_id], &cur_second))).collect();
        let max_distance = r_distances.iter().max_by_key(|x| x.1).unwrap().1;
        for r_distance in r_distances {
            if r_distance.1 == max_distance {
                scores[r_distance.0] += 1;
            }
        };
        cur_second += 1;
    }
    

    println!("{:?}", max_distance);
    println!("{:?}", scores.iter().max().unwrap());
}
