mod polymer;

use std::io::Read;
use polymer::Polymer;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").unwrap();
    let mut input = Vec::new();
    f.read_to_end(&mut input).unwrap();
    let mut state = Polymer { data: input };
    let mut start = 0;
    let mut i = 0;
    while state.data.len() > 10 {
        if i % 1000 == 0 {
            println!("start: {}, length: {}", start, state.data.len());
        }
        i += 1;
        start = state.react(start);
    }
}
