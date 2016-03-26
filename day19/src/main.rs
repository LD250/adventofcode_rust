use std::io::prelude::*;
use std::fs::File;

fn main() {

    fn get_new_molecules(init_molecule: String, from: &str, to: &str) -> Vec<String> {
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

    let replaced_molecule = get_new_molecules("HOHOH".to_string(), "H", "OO");


    println!("{:?}", replaced_molecule);
}
