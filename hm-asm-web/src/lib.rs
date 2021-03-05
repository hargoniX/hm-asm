use pest::Parser;
use wasm_bindgen::prelude::*;

use hm_asm_simulator::{
    generate::generate_binary,
    parse::{parse_asm, AsmParser, Rule},
};


#[wasm_bindgen]
pub fn simulate(code: &str, cycles: usize) -> JsValue {
    let instructions = parse_asm(
        AsmParser::parse(Rule::program, &code).unwrap_or_else(|e| panic!("{}", e)),
    );
    let states = hm_asm_simulator::simulate::simulate(instructions, cycles);

    JsValue::from_serde(&states).unwrap()
}


#[wasm_bindgen]
pub fn assemble(code: &str) -> JsValue {
    let instructions = parse_asm(
        AsmParser::parse(Rule::program, &code).unwrap_or_else(|e| panic!("{}", e)),
    );
    let binary = generate_binary(instructions);
    JsValue::from_serde(&binary).unwrap()
}
