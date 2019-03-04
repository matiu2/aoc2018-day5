mod polymer;
use polymer::Polymer;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut state = Polymer { data: input };
    while state.data.len() > 10 {
        println!("length: {}", state.data.len());
        state = state.react();
    }
}
