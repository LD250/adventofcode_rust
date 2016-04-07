// #![feature(slice_patterns)]
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {

    let mut content = String::new();

    match File::open("data/input.txt") {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
            content = content.trim().replace(",", "");
        }
    };

    let lines: Vec<&str> = content.split("\n").collect();
    #[derive(Debug)]
    struct Operation<'a> {
        reg: &'a str,
        instr: &'a str,
        value: i32,
    };

    let mut operations: Vec<Operation> = Vec::new();

    for line in &lines {
        let ops: Vec<&str> = line.split(' ').collect();
        let op = match ops[0] {
            r @ "jmp" => {
                Operation {
                    reg: "",
                    instr: r,
                    value: ops[1].parse().expect(ops[1]),
                }
            }
            r @ "jio" | r @ "jie" => {
                Operation {
                    reg: ops[1],
                    instr: r,
                    value: ops[2].parse().expect(ops[2]),
                }
            }
            r => {
                Operation {
                    reg: ops[1],
                    instr: r,
                    value: 0,
                }
            }
        };
        operations.push(op);
    }

    fn run_program(reg_a_value: i32, operations: &Vec<Operation>) {
        let mut regs = HashMap::new();
        regs.insert("b", 0);
        regs.insert("a", reg_a_value);
        let mut i: usize = 0;

        while i < operations.len() {
            match operations[i] {

                Operation { reg, instr, value } => {
                    let mut plus: i32 = 1;
                    match instr {
                        "jmp" => plus = value,
                        other => {
                            let reg_value = match regs.get(&reg) {
                                Some(&val) => val,
                                _ => panic!("register not found"),
                            };
                            match other {
                                "hlf" => {
                                    regs.insert(reg, reg_value / 2);
                                }
                                "tpl" => {
                                    regs.insert(reg, reg_value * 3);
                                }
                                "inc" => {
                                    regs.insert(reg, reg_value + 1);
                                }
                                "jie" => {
                                    if reg_value % 2 == 0 {
                                        plus = value
                                    }
                                }
                                "jio" => {
                                    if reg_value == 1 {
                                        plus = value;
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
    };
    run_program(0, &operations);
    run_program(1, &operations);
}
