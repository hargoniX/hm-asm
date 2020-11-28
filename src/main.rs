use std::env;
use std::fs;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

mod asm;
mod generate;
mod parse;

use parse::{parse_asm, AsmParser};

fn main() {
    let file_name = env::args().nth(1);

    let file_content = match file_name {
        Some(file_name) => {
            fs::read_to_string(file_name).expect("Could not read the provided asm file")
        }
        None => {
            println!("No input file was provided");
            return;
        }
    };

    let instructions = parse_asm(
        AsmParser::parse(parse::Rule::program, &file_content).unwrap_or_else(|e| panic!("{}", e)),
    );
    println!("{:#?}", instructions);
}
