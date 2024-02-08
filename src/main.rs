mod types;

use types::*;

fn main() {
    let input = MinusOne::<0>::new();
    let output: usize = input.into();
    println!("output: {:?}", output);
}
