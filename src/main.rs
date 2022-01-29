use std::env;
use rdkitcffi::{Molecule, read_sdfile};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sd_filename = &args[1];
    println!("filename is {}", sd_filename);
    let mut mol_opt_list: Vec<Option<Molecule>> = read_sdfile(sd_filename);
    let mut mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
    mol_list.iter_mut().for_each(|m| m.remove_all_hs());
    for m in mol_list {
        let desc = m.get_descriptors();
        println!("{}", desc)
    }
    println!("Done");
}
