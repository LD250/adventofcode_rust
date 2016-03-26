use std::env;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use std::collections::HashSet;

use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


fn get_new_molecules(init_molecule: &str, from: &str, to: &str) -> Vec<String> {
    println!("{:?}", init_molecule);
    let splited_molecule: Vec<&str> = init_molecule.split(from).collect();
    let molecule_len = splited_molecule.len();
    (0..molecule_len - 1).map(|i| splited_molecule.iter()
                                                    .enumerate()
                                                    .map(|(splited_index, &splited_val)| {
                                                            if splited_index == i {
                                                                splited_val.to_string() + to
                                                            } else if splited_index != molecule_len - 1 {
                                                                splited_val.to_string() + from
                                                            } else {
                                                                splited_val.to_string()
                                                            }
                                                        }).collect::<Vec<String>>().concat())
                        .collect()


}

fn apply_all_replacements(init_molecule: &str, replacements: Vec<(&str, &str)>) -> HashSet<String> {
    replacements.iter().map(|&(from, to)| get_new_molecules(init_molecule, from, to)).flat_map(|molecula| molecula).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const DEFAULT_THREADS_COUNT: usize = 4;

    let threads_count = match args.len() {
        1 => DEFAULT_THREADS_COUNT, 
        _ => match args[1].parse() {
            Ok(c) if c <= 20 => c,
            _ => panic!("Argument must be an integer that less than 20"),
        }
    };

    let path = Path::new("data/input.txt");
    let mut content = String::new();

    match File::open(path) {
        Err(_) => panic!("Error reading file"),
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        },
    };
    let lines: Vec<&str> = content.trim().split("\n").collect();
    let replacements_len = lines.len() - 2;
    let molecule = lines[replacements_len + 1];
    let replacements: Vec<(&str, &str)> = lines.iter()
                                               .take(replacements_len)
                                               .map(|&line| {
                                                        let parts: Vec<&str> = line.split(" => ").collect();
                                                        (parts[0], parts[1])})
                                               .collect(); 

    //let (tx, rx): (Sender<HashSet<String>>, Receiver<HashSet<String>>) = mpsc::channel();
    for id in 0..threads_count {
        //let thread_tx = tx.clone();
        let repl: Vec<(&str, &str)> = replacements.clone().iter().map(|&x| x).collect();//.skip(0).take(5).collect();
        thread::spawn(move || {
            let molecules = apply_all_replacements(molecule, repl);
            //thread_tx.send(molecules).unwrap();
            println!("thread {} finished", 34);
        });
    }
    println!("{:?}", &replacements[1..threads_count]);
    println!("{:?}", molecule);
 
    let replaced_molecule = get_new_molecules("HOHOH", "H", "OO");
    
    //let handle = thread::spawn(|| {
    //    get_new_molecules("HOHOHHHHOHHHHHHHHHHOOOOOOOOOOOHHHHHHHHHHOOOOOOOOOO", "H", "OO")
    //});

    //println!("{:?}", handle.join().unwrap());
    //thread::sleep(Duration::from_millis(50));
    //
/*
    let mut result_molecules: HashSet<String>;
    for _ in 0..threads_count {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there no messages available
        rx.recv().unwrap();
    }
*/
    /*println!("{:?}", result_molecules);
    println!("{:?}", result_molecules.len());*/
}
