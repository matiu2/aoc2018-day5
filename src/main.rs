mod polymer;

use std::io::Read;
use polymer::Polymer;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = Vec::new();
    f.read_to_end(&mut input).unwrap();
    let mut state = Polymer { data: input };
    while state.data.len() > 10 {
        println!("length: {}", state.data.len());
        state.react();
    }
}
