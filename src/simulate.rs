use crate::generate::insert_label;
use crate::asm::*;
use crate::generate::generate_binary;

use std::collections::HashMap;
use std::fmt;

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


impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "<tr>")?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.step)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.clk as u8)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.pc)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.addr_bus)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.data_bus)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.ir)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.dr)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">{}</td>", self.akku)?;
        writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">C: {}, Z: {}, N: {}</td>", self.sr.carry as u8, self.sr.zero as u8, self.sr.negative as u8)?;
        if let Some(opcode_info) = self.opcode_info {
            writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\">addr: {}, val: {}</td>", opcode_info.addr, opcode_info.content)?;
        } else {
            writeln!(f, "<td style=\"border: 1px solid #000000; padding: 0mm 1.91mm;\"></td>")?;
        }
        writeln!(f, "</tr>")
    }
}


pub fn simulate<'a>(instructions: Vec<Instruction<'a>>, max_steps: usize) -> Vec<State> {
    let mut data_memory = generate_binary(instructions.clone()).data_memory;

    let mut labels: HashMap<&str, u8> = HashMap::new();

    let mut states: Vec<State> = Vec::new();
    let mut step: usize = 0;
    let mut clk: bool;
    let mut pc: u8 = 0;
    let mut addr_bus: u8;
    let mut data_bus: u8;
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

    let mut next_pc: Option<u8> = None;
    let mut next_akku = 0;
    let mut next_carry = false;
    let mut next_data_mem_addr: Option<usize> = None;
    let mut next_data_mem_val: Option<u8> = None;
    loop {
        if next_akku != akku {
            next_carry = (next_akku & (1<<4)) != 0;
        }

        if let Some(mut next_pc_value) = next_pc {
            next_pc_value = next_pc_value % 16;
            next_akku = next_akku % 16;
            pc = next_pc_value;
            next_pc = None;
        }


        akku = next_akku;

        if let Some(addr) = next_data_mem_addr {
            data_memory[addr] = next_data_mem_val.unwrap();
            next_data_mem_addr = None;
            next_data_mem_val = None;
        }

        let instruction = if pc as usize > instructions.len() - 1 {
            Instruction::NoArgumentInstruction(NoArgumentInstruction::NOP, None)
        } else {
            instructions[pc as usize]
        };

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

        sr.carry = next_carry;
        sr.zero = akku == 0 || akku == (1<<4);
        sr.negative = (akku & (1<<3)) != 0;

        dr = binary_instruction.argument;
        ir = binary_instruction.opcode;

        addr_bus = dr;

        match instruction {
            Instruction::NoArgumentInstruction(instruction, _) => match instruction {
                NoArgumentInstruction::NOP => {}
            },
            Instruction::ConstantArgumentInstruction(instruction, _) => match instruction {
                ConstantArgumentInstruction::BRC(arg) if sr.carry => next_pc = Some(pc + arg),
                ConstantArgumentInstruction::BRN(arg) if sr.negative => next_pc = Some(pc + arg),
                ConstantArgumentInstruction::BRZ(arg) if sr.zero => next_pc = Some(pc + arg),
                _ => {}
            },
            Instruction::Jump(arg, _) => match arg {
                JumpArgument::Label(label) => {
                    next_pc = Some(*labels.get(label).unwrap());
                    addr_bus = *labels.get(label).unwrap();
                },
                JumpArgument::Location(location) => {
                    next_pc = Some(location);
                    addr_bus = location;
                },
            },
            Instruction::MemoryLocationInstruction(arg, _) => match arg {
                MemoryLocationInstruction::STA(arg) => {
                    next_data_mem_addr = Some(arg as usize);
                    next_data_mem_val = Some(akku);
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
                    Argument::MemoryLocation(location) => next_akku = akku + (data_memory[location as usize] ^ 0b1111) + 1,
                    Argument::Constant(val) => next_akku = akku + (val ^ 0b1111) + 1
                }
            }
        }

        data_bus = data_memory[addr_bus as usize];

        if let None = next_pc {
            next_pc = Some(pc + 1);
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

        step += 1;
        if step == max_steps {
            break;
        }
    }

    return states;
}
