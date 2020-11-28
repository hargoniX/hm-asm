use crate::generate::insert_label;
use crate::asm::*;
use crate::generate::generate_binary;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StateRegister {
    carry: bool,
    zero: bool,
    negative: bool
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OpcodeInfo {
    addr: u8,
    content: u8
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct State {
    step: usize,
    clk: bool,
    pc: u8,
    addr_bus: u8,
    data_bus: u8,
    ir: u8, // instruction register
    dr: u8, // data register
    akku: u8,
    sr: StateRegister,
    opcode_info: Option<OpcodeInfo>
}

pub fn simulate<'a>(instructions: Vec<Instruction<'a>>, max_steps: usize) -> Vec<State> {
    let mut data_memory = generate_binary(instructions.clone()).data_memory;

    let mut labels: HashMap<&str, u8> = HashMap::new();

    let mut states: Vec<State> = Vec::new();
    let mut step: usize = 0;
    let mut clk: bool = false;
    let mut pc: u8 = 0;
    let mut addr_bus: u8 = 0;
    let mut data_bus: u8 = 0;
    let mut ir : u8 = 0;
    let mut dr : u8 = 0;
    let mut akku : u8 = 0;
    let mut sr: StateRegister = StateRegister {
        carry: false,
        zero: false,
        negative: false
    };

    for instruction in instructions.iter() {
        match instruction {
            Instruction::NoArgumentInstruction(_, label)
            | Instruction::MemoryLocationInstruction(_, label)
            | Instruction::ConstantArgumentInstruction(_, label)
            | Instruction::ArgumentInstruction(_, label)
            | Instruction::Jump(_, label) => {
                insert_label(&mut labels, label);
            }
        }
    }

    let mut next_pc = 0;
    let mut next_akku = 0;
    let mut next_data_mem_addr = 0;
    let mut next_data_mem_val = 0;
    let mut first_iter = true;
    loop {
        pc = next_pc;
        akku = next_akku;
        if !first_iter {
            data_memory[next_data_mem_addr] = next_data_mem_val;
        }
        let instruction = instructions[pc as usize];

        let binary_instruction: BinaryInstruction = match instruction {
            Instruction::NoArgumentInstruction(instruction, _) => instruction.into(),
            Instruction::MemoryLocationInstruction(instruction, _) => instruction.into(),
            Instruction::ConstantArgumentInstruction(instruction, _) => instruction.into(),
            Instruction::ArgumentInstruction(instruction, _) => instruction.into(),
            Instruction::Jump(argument, _) => match argument {
                JumpArgument::Location(arg) => BinaryInstruction {
                    opcode: 8,
                    argument: arg,
                },
                JumpArgument::Label(arg) => {
                    if let Some(address) = labels.get(arg) {
                        BinaryInstruction {
                            opcode: 8,
                            argument: *address,
                        }
                    } else {
                        panic!("Tried to JMP to label: {}, which does not exist", arg);
                    }
                }
            },
        };

        clk = false;
        addr_bus = pc;
        data_bus = data_memory[pc as usize];
        let opcode_info = match instruction {
            Instruction::MemoryLocationInstruction(instruction, _) => {
                match instruction {
                    MemoryLocationInstruction::STA(location) => Some(OpcodeInfo{addr: location, content: data_memory[location as usize]}),
                }
            },
            Instruction::ArgumentInstruction(instruction, _) => {
                match instruction {
                    ArgumentInstruction::ADD(argument) | ArgumentInstruction::SUB(argument) | ArgumentInstruction::LDA(argument) => match argument {
                        Argument::MemoryLocation(location) => Some(OpcodeInfo{addr: location, content: data_memory[location as usize]}),
                        _ => None
                    }
                }
            }
            _ => None
        };
        states.push(State{
            step,
            clk,
            pc,
            addr_bus,
            data_bus,
            ir,
            dr,
            akku,
            sr,
            opcode_info
        });

        clk = true;
        dr = binary_instruction.argument;
        ir = binary_instruction.opcode;

        match instruction {
            Instruction::NoArgumentInstruction(instruction, _) => match instruction {
                NoArgumentInstruction::NOP => {}
            },
            Instruction::ConstantArgumentInstruction(instruction, _) => match instruction {
                ConstantArgumentInstruction::BRC(arg) if sr.carry => next_pc = pc + arg,
                ConstantArgumentInstruction::BRN(arg) if sr.negative => next_pc = pc + arg,
                ConstantArgumentInstruction::BRZ(arg) if sr.zero => next_pc = pc + arg,
                _ => {}
            },
            Instruction::Jump(arg, _) => match arg {
                JumpArgument::Label(label) => {
                    next_pc = *labels.get(label).unwrap();
                    addr_bus = next_pc;
                },
                JumpArgument::Location(location) => {
                    next_pc = location;
                    addr_bus = next_pc;
                },
            },
            Instruction::MemoryLocationInstruction(arg, _) => match arg {
                MemoryLocationInstruction::STA(arg) => {
                    next_data_mem_addr = arg as usize;
                    next_data_mem_val = akku;
                }
            },
            Instruction::ArgumentInstruction(instruction, _) => match instruction {
                ArgumentInstruction::LDA(arg) => match arg {
                    Argument::MemoryLocation(location) => next_akku = data_memory[location as usize],
                    Argument::Constant(val) => next_akku = val
                },
                ArgumentInstruction::ADD(arg) => match arg {
                    Argument::MemoryLocation(location) => next_akku = akku + data_memory[location as usize],
                    Argument::Constant(val) => next_akku = akku + val
                },
                ArgumentInstruction::SUB(arg) => match arg {
                    Argument::MemoryLocation(location) => next_akku = akku - data_memory[location as usize],
                    Argument::Constant(val) => next_akku = akku - val
                }
            }
        }

        if next_pc == pc {
            next_pc = pc + 1;
        }

        states.push(State{
            step,
            clk,
            pc,
            addr_bus,
            data_bus,
            ir,
            dr,
            akku,
            sr,
            opcode_info
        });

        sr.carry = (akku & (1<<4)) != 0;
        sr.zero = akku == 0 || akku == (1<<4);
        sr.negative = (akku & (1<<3)) != 0;

        step += 1;
        first_iter = false;
        if step == max_steps {
            break;
        }
    }

    return states;
}
