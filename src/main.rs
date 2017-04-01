mod token;
mod tokeniser;
use std::io::{self, Read};

fn main() {
    let mut inbuf = String::new();
    io::stdin().read_to_string(&mut inbuf).unwrap();
    let tokens  = tokeniser::tokenise(&inbuf);
    for token in &tokens {
        println!("{:?}", token);
    }
}
