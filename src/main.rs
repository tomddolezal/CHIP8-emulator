use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        0 | 1 => "games/MISSILE",
        _ => args.get(1).unwrap(),
    };
    let mut file = File::open(file_name).unwrap();
    let mut data = Vec::<u8>::new();
    // Put game in data
    file.read_to_end(&mut data).expect("Error: Game file not found.");
 

}

