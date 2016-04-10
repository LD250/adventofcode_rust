extern crate itertools;

use std::io::prelude::*;
use std::fs::File;
use itertools::Itertools;

fn calculate_qe(weights: &Vec<u16>, groups_num: u16, start_from: usize) -> u64 {
    let sum: u16 = weights.iter().fold(0, |sum, &w| sum + w);
    let group_weight = sum / groups_num;
    let qe: u64 = (start_from..weights.len())
                      .map(|w| {
                          let groups: Vec<Vec<&u16>> = weights.iter()
                                                              .combinations_n(w)
                                                              .filter(|w_list: &Vec<&u16>| w_list.iter().fold(0, |sum, &w| sum + w) == group_weight)
                                                              .collect();
                          match groups.iter()
                                      .map(|w_list: &Vec<&u16>| w_list.iter().fold(1, |pro, &&w| pro * w as u64))
                                      .min() {
                              Some(m) => m,
                              _ => 0,
                          }
                      })
                      .skip_while(|&x| x == 0)
                      .take(1)
                      .collect::<Vec<u64>>()[0];
    qe

}

fn main() {

    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
    };
    type Scheme = Vec<u16>;

    let weights: Vec<u16> = content.trim().split("\n").map(|w| w.parse().unwrap()).collect();
    let qe: u64 = calculate_qe(&weights, 3, 5);
    println!("{}", qe);
    let qe: u64 = calculate_qe(&weights, 4, 4);
    println!("{}", qe);

}
