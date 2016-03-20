use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
            content = content.trim().replace(".", "");
        },
    };

    let lines: Vec<&str> = content.split("\n").collect();
    let mut guests_relation = HashMap::new();
    let mut guests = HashSet::new();
    for line in &lines{
        // Alice would gain 26 happiness units by sitting next to Carol.
        // Alice would lose 82 happiness units by sitting next to David.
        let guest_info: Vec<&str> = line.split(" ").collect();
        let happiness: i32 = guest_info[3].parse().expect(guest_info[3]);
        guests_relation.insert((guest_info[0], guest_info[10]), if guest_info[2] == "gain" {happiness} else {0 - happiness});
        guests.insert(guest_info[0]);
        //guests.insert((guest_info[0], guest_info[10]), happiness);
    }
    // println!("{:?}", guests);

    fn update_sitted_guests<'a>(sg: &Vec<&'a str>, ng: &'a str) -> Vec<&'a str> {
        let mut nsg = sg.clone();
        nsg.push(ng);
        nsg
    }

    fn calculate_sitting_arrangement(guests_relation: &HashMap<(&str, &str), i32>, guests: &HashSet<&str>, sitted_guests: Vec<&str>) -> i32 {
       let available_guests: Vec<&str> = guests.into_iter().filter(|&g| !sitted_guests.contains(g)).cloned().collect(); 
       match available_guests.is_empty() {
           false => {
               available_guests.iter().map(|g| calculate_sitting_arrangement(guests_relation, guests, update_sitted_guests(&sitted_guests, g))).max().unwrap()
           },
           true => {
               //println!("{:?}", sitted_guests);
               (0..(sitted_guests.len() - 2))
                   .map(|i: usize| guests_relation.get(&(sitted_guests[i], sitted_guests[i+1])).unwrap()
                            + guests_relation.get(&(sitted_guests[i+1], sitted_guests[i])).unwrap()).fold(0, |sum, x| sum + x) + 
                                guests_relation.get(&(sitted_guests[sitted_guests.len()-1], sitted_guests[0])).unwrap()
                                + guests_relation.get(&(sitted_guests[0], sitted_guests[sitted_guests.len()-1])).unwrap()
               }

       }
    
    }

    let best_a = calculate_sitting_arrangement(&guests_relation, &guests, vec!());
    println!("{:?}", best_a);
}
