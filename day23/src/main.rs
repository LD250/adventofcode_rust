// #![feature(slice_patterns)]
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {

    let mut content = String::new();

    let lines: Vec<&str>;

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
            lines = content.trim().split("\n").collect();
        }
    };

    let mut regs = HashMap::new();
    regs.insert("a", 1);
    regs.insert("b", 0);

    let mut i: usize = 0;

    let mut out = false;

    while !out {
        match lines.get(i) {
            None => {
                out = true;
            }

            Some(ref s) => {
                let operations: Vec<&str> = s.splitn(2, ' ').collect();
                let args: Vec<&str> = operations[1].split(", ").collect();
                let mut plus: i32 = 1;
                match operations[0] {
                    "jmp" => {
                        plus = args[0].parse().expect(args[0]);
                    }
                    other => {
                        let reg_value = match regs.get(&args[0]) {
                            Some(&val) => val,
                            _ => panic!("register not found"),
                        };
                        match other {
                            "hlf" => {
                                regs.insert(&args[0], reg_value / 2);
                            }
                            "tpl" => {
                                regs.insert(&args[0], reg_value * 3);
                            }
                            "inc" => {
                                regs.insert(&args[0], reg_value + 1);
                            }
                            "jie" => {
                                plus = if reg_value % 2 == 0 {
                                    args[1].parse().expect(args[1])
                                } else {
                                    1
                                };
                            }
                            "jio" => {
                                if reg_value == 1 {
                                    plus = args[1].parse().expect(args[1]);
                                };
                            }
                            _ => panic!("out"),
                        };
                    }
                };
                plus = i as i32 + plus;
                i = plus as usize;
            }
        }
    }
    println!("{:?}", regs);
}
