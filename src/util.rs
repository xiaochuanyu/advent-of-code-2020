use std::io::BufReader;

use std::fs::File;

pub fn read_input() -> BufReader<File> {
    let file = File::open("input.txt").expect("cannot open file");
    BufReader::new(file)
}
