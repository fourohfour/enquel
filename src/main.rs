mod token;
mod tokeniser;

mod parse;
mod parser;

use std::io::{self, Read};

fn main() {
    let mut inbuf = String::new();
    io::stdin().read_to_string(&mut inbuf).unwrap();
    let tokens  = tokeniser::tokenise(&inbuf);
    let tree    = parser::parse(&tokens); 
}
