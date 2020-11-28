use crate::asm::*;
use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "asm.pest"]
pub struct AsmParser;

pub fn parse_asm<'a>(pairs: Pairs<'a, Rule>) -> Vec<Instruction<'a>> {
    let mut instruction = Vec::new();
    let mut instruction_counter = 0;
    for stmnt in pairs {
        if let Rule::stmt = stmnt.as_rule() {
            let mut stmnt = stmnt.into_inner();
            let first = stmnt.next();
            let second = stmnt.next();

            // Second can only be Some if we have a label, thus second must be the
            // instruction if that is the case.
            if second.is_some() {
                instruction.push(parse_instruction(second.unwrap(), first, instruction_counter));
            } else {
                instruction.push(parse_instruction(first.unwrap(), None, instruction_counter));
            }

            instruction_counter += 1;
        }

        if instruction_counter > 15 {
            panic!("This program contains more than 16 instructions, that is impossible on this processor");
        }
    }
    instruction
}

fn parse_instruction<'a>(instruction: Pair<'a, Rule>, label: Option<Pair<'a, Rule>>, instruction_counter: u8) -> Instruction<'a>{
    let label = label.map(|l| parse_label(l, instruction_counter));
    let mut instruction = instruction.into_inner();
    let mnemonic = instruction.next().unwrap();

    match mnemonic.as_rule() {
        Rule::no_arg_instruction => parse_no_arg_instruction(mnemonic, label),
        Rule::arg_instruction => {
            parse_arg_instruction(mnemonic, instruction.next().unwrap(), label)
        },
        Rule::jump_instruction => {
            parse_jump_instruction(mnemonic, instruction.next().unwrap(), label)
        },
        Rule::memory_location_instruction => {
            parse_memory_location_instruction(mnemonic, instruction.next().unwrap(), label)
        },
        Rule::constant_arg_instruction => {
            parse_constant_arg_instruction(mnemonic, instruction.next().unwrap(), label)
        },
        _ => unreachable!()
    }
}

fn parse_no_arg_instruction<'a>(instruction: Pair<'a, Rule>, label: Option<Label<'a>>) -> Instruction<'a> {
    match instruction.as_str() {
        "NOP" => Instruction::NoArgumentInstruction(NoArgumentInstruction::NOP, label),
        _ => unreachable!()
    }
}

fn parse_arg_instruction<'a>(instruction: Pair<'a, Rule>, arg: Pair<'a, Rule>, label: Option<Label<'a>>) -> Instruction<'a> {
    let arg = parse_argument(arg);
    match instruction.as_str() {
        "LDA" => Instruction::ArgumentInstruction(ArgumentInstruction::LDA(arg), label),
        "ADD" => Instruction::ArgumentInstruction(ArgumentInstruction::ADD(arg), label),
        "SUB" => Instruction::ArgumentInstruction(ArgumentInstruction::SUB(arg), label),
        _ => unreachable!()
    }
}

fn parse_jump_instruction<'a>(instruction: Pair<'a, Rule>, arg: Pair<'a, Rule>, label: Option<Label<'a>>) -> Instruction<'a> {
    let arg = parse_jump_argument(arg);
    match instruction.as_str() {
        "JMP" => Instruction::Jump(arg, label),
        _ => unreachable!()
    } 
}

fn parse_memory_location_instruction<'a>(instruction: Pair<'a, Rule>, arg: Pair<'a, Rule>,label: Option<Label<'a>>) -> Instruction<'a> {
    let arg_string = arg.as_str();
    let arg_value = u8::from_str_radix(&arg_string[1..arg_string.len()-1], 16).unwrap();
    match instruction.as_str() {
        "STA" => Instruction::MemoryLocationInstruction(MemoryLocationInstruction::STA(arg_value), label),
        _ => unreachable!()
    }
}

fn parse_constant_arg_instruction<'a>(instruction: Pair<'a, Rule>, arg: Pair<'a, Rule>, label: Option<Label<'a>>) -> Instruction<'a> {
    let arg_value = u8::from_str_radix(&arg.as_str()[1..], 16).unwrap();
    match instruction.as_str() {
        "BRZ" => Instruction::ConstantArgumentInstruction(ConstantArgumentInstruction::BRZ(arg_value), label),
        "BRC" => Instruction::ConstantArgumentInstruction(ConstantArgumentInstruction::BRC(arg_value), label),
        "BRN" => Instruction::ConstantArgumentInstruction(ConstantArgumentInstruction::BRN(arg_value), label),
        _ => unreachable!()
    }
}

fn parse_argument<'a>(argument: Pair<'a, Rule>) -> Argument {
    let argument = argument.into_inner().next().unwrap();
    match argument.as_rule() {
        Rule::memory_location => {
            let arg_string = argument.as_str();
            let arg_value = u8::from_str_radix(&arg_string[1..arg_string.len()-1], 16).unwrap();
            Argument::MemoryLocation(arg_value)
        },
        Rule::digit_literal => {
            let arg_value = u8::from_str_radix(&argument.as_str()[1..], 16).unwrap();
            Argument::Constant(arg_value)
        },
        _ => unreachable!()
    }
}

fn parse_label<'a>(label: Pair<'a, Rule>, instruction_counter: u8) -> Label<'a> {
    match label.as_rule() {
        Rule::label => {
            Label {
                name: label.as_str(),
                location: instruction_counter
            }
        },
        _ => unreachable!()
    }
}

fn parse_jump_argument<'a>(arg: Pair<'a, Rule>) -> JumpArgument<'a> {
    let arg = arg.into_inner().next().unwrap();
    match arg.as_rule() {
        Rule::label => JumpArgument::Label(arg.as_str()),
        Rule::jump_location => JumpArgument::Location(u8::from_str_radix(arg.as_str(), 16).unwrap()),
        _ => unreachable!()
    }
}
