use std::env;
use std::fs;

use hm_asm_simulator::{
    generate::generate_binary,
    parse::{parse_asm, AsmParser, Rule},
    simulate::simulate
};

use pest::Parser;

mod html;
use html::html_state_table;


fn main() {
    let sub_cmd = env::args().nth(1);
    let file_name = env::args().nth(2);
    let steps = env::args().nth(3);

    if let Some(sub_cmd) = sub_cmd {
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
            AsmParser::parse(Rule::program, &file_content).unwrap_or_else(|e| panic!("{}", e)),
        );

        if sub_cmd == "generate" {
            let binary = generate_binary(instructions);
            println!("{}", binary);
        }
        else if sub_cmd == "simulate" {
            if let Some(steps) = steps {
                let states = simulate(instructions, steps.parse::<usize>().unwrap());
                //println!("{:#?}", states);
                println!("{}", html_state_table(states));
            }
        }
    } else {
        println!("No argument was passed, exiting");
    }
}
