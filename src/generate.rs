use crate::asm::*;
use std::collections::HashMap;
use std::fmt;

pub struct Program {
    data_memory: [u8; 16],
    program_memory: [u8; 16],
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Data Memory:")?;
        for chunk in self.data_memory.chunks(4) {
            writeln!(
                f,
                "{:x} {:x} {:x} {:x}",
                chunk[0], chunk[1], chunk[2], chunk[3]
            )?;
        }

        writeln!(f, "Program Memory:")?;
        for chunk in self.program_memory.chunks(4) {
            writeln!(
                f,
                "{:x} {:x} {:x} {:x}",
                chunk[0], chunk[1], chunk[2], chunk[3]
            )?;
        }
        writeln!(f, "And that's your program!")
    }
}

pub fn generate_binary(instructions: Vec<Instruction>) -> Program {
    let mut labels: HashMap<&str, u8> = HashMap::new();
    let mut data_memory: [u8; 16] = [0; 16];
    let mut program_memory: [u8; 16] = [0; 16];

    // collect all labels
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

    for (c, instruction) in instructions.iter().enumerate() {
        let binary_instruction: BinaryInstruction = match instruction {
            Instruction::NoArgumentInstruction(instruction, _) => (*instruction).into(),
            Instruction::MemoryLocationInstruction(instruction, _) => (*instruction).into(),
            Instruction::ConstantArgumentInstruction(instruction, _) => (*instruction).into(),
            Instruction::ArgumentInstruction(instruction, _) => (*instruction).into(),
            Instruction::Jump(argument, _) => match argument {
                JumpArgument::Location(arg) => BinaryInstruction {
                    opcode: 8,
                    argument: *arg,
                },
                JumpArgument::Label(arg) => {
                    if let Some(address) = labels.get(*arg) {
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

        program_memory[c] = binary_instruction.opcode;
        data_memory[c] = binary_instruction.argument;
    }

    Program {
        data_memory,
        program_memory,
    }
}

fn insert_label<'a>(hashmap: &mut HashMap<&'a str, u8>, label: &Option<Label<'a>>) {
    if let Some(label) = label {
        hashmap.insert(label.name, label.location);
    }
}
