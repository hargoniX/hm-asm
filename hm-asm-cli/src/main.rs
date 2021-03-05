extern crate clap;

use std::fs;

use hm_asm_simulator::{
    generate::generate_binary,
    parse::{parse_asm, AsmParser, Rule},
    simulate::simulate
};

use pest::Parser;
use clap::{Arg, App, SubCommand};

mod html;
use html::html_state_table;


fn main() {
    let matches = App::new("hm-asm-cli")
        .version("0.1.0")
        .author("Henrik Boeving <henrik@boeving-net.de")
        .about("A CLI frontend for the hm-asm-simulator")
        .subcommand(SubCommand::with_name("compile")
            .arg(Arg::with_name("COMP_FILE")
                .help("Sets the asm file to compile")
                .value_name("FILE") 
                .takes_value(true)))
        .subcommand(SubCommand::with_name("simulate")
            .arg(Arg::with_name("SIM_FILE")
                .help("Sets the asm file to simulate")
                .value_name("FILE") 
                .takes_value(true))
            .arg(Arg::with_name("cycles")
                .help("How many cycles to run the simulator for")
                .value_name("cycles")
                .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let file_content = fs::read_to_string(matches.value_of("COMP_FILE").unwrap()).expect("Could not read the provided asm file");
        let instructions = parse_asm(AsmParser::parse(Rule::program, &file_content).unwrap_or_else(|e| panic!("{}", e)));
        let binary = generate_binary(instructions);
        println!("{}", binary);
    } else if let Some(matches) = matches.subcommand_matches("simulate") {
        let file_content = fs::read_to_string(matches.value_of("SIM_FILE").unwrap()).expect("Could not read the provided asm file");
        let instructions = parse_asm(AsmParser::parse(Rule::program, &file_content).unwrap_or_else(|e| panic!("{}", e)));
        let states = simulate(instructions, matches.value_of("cycles").unwrap().parse::<usize>().unwrap());
        //println!("{:#?}", states);
        println!("{}", html_state_table(states));
    }
}
