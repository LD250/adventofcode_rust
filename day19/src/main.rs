use std::env;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use std::collections::HashSet;

use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::Arc;


fn get_new_molecules(init_molecule: &str, from: &str, to: &str) -> Vec<String> {
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

fn apply_all_replacements(init_molecule: &str, replacements: &[(String, String)]) -> HashSet<String> {
    replacements.iter().map(|repl| get_new_molecules(init_molecule, &repl.0, &repl.1)).flat_map(|molecula| molecula).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const DEFAULT_THREADS_COUNT: usize = 4;

    let threads_count = match args.len() {
        1 => DEFAULT_THREADS_COUNT, 
        _ => match args[1].parse() {
            Ok(c) if c <= 43 => c,
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
    let molecule = lines[replacements_len + 1].to_string();
    let replacements: Vec<(String, String)> = lines.iter()
                                               .take(replacements_len)
                                               .map(|&line| {
                                                        let parts: Vec<&str> = line.split(" => ").collect();
                                                        (parts[0].to_string(), parts[1].to_string())})
                                               .collect(); 

    let (tx, rx): (Sender<HashSet<String>>, Receiver<HashSet<String>>) = mpsc::channel();
    let shared_repl = Arc::new(replacements.clone());
    let chunks_per_thread = replacements_len / threads_count;
    for id in 0..threads_count {
        let thread_tx = tx.clone();
        let repl = shared_repl.clone();
        let mol = molecule.clone();
        let last_id = if id == threads_count - 1 {replacements_len} else { (id + 1) * chunks_per_thread};
        thread::spawn(move || {
            let molecules = apply_all_replacements(&mol, &repl[id * chunks_per_thread..last_id]);
            thread_tx.send(molecules).unwrap();
            println!("thread {} finished", id);
        });
    }
 
    let mut result_molecules: HashSet<String> = HashSet::new();
    for _ in 0..threads_count {
        let molecules = rx.recv().unwrap();
        //println!("{:?}", molecules);
        result_molecules = result_molecules.union(&molecules).cloned().collect();
    }

    println!("{:?}", result_molecules.len());
}
