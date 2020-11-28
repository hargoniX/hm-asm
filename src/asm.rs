#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Argument {
    MemoryLocation(u8),
    Constant(u8),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Label<'a> {
    pub name: &'a str,
    pub location: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JumpArgument<'a> {
    Location(u8),
    Label(&'a str),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction<'a> {
    NoArgumentInstruction(NoArgumentInstruction, Option<Label<'a>>),
    MemoryLocationInstruction(MemoryLocationInstruction, Option<Label<'a>>),
    ConstantArgumentInstruction(ConstantArgumentInstruction, Option<Label<'a>>),
    ArgumentInstruction(ArgumentInstruction, Option<Label<'a>>),
    Jump(JumpArgument<'a>, Option<Label<'a>>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NoArgumentInstruction {
    NOP,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryLocationInstruction {
    STA(u8),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConstantArgumentInstruction {
    BRZ(u8),
    BRC(u8),
    BRN(u8),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArgumentInstruction {
    LDA(Argument),
    ADD(Argument),
    SUB(Argument),
}

pub struct BinaryInstruction {
    pub opcode: u8,
    pub argument: u8,
}

impl<'a> Into<BinaryInstruction> for NoArgumentInstruction {
    fn into(self) -> BinaryInstruction {
        match self {
            NoArgumentInstruction::NOP => BinaryInstruction {
                opcode: 0,
                argument: 0,
            },
        }
    }
}

impl<'a> Into<BinaryInstruction> for MemoryLocationInstruction {
    fn into(self) -> BinaryInstruction {
        match self {
            MemoryLocationInstruction::STA(arg) => BinaryInstruction {
                opcode: 3,
                argument: arg,
            },
        }
    }
}

impl<'a> Into<BinaryInstruction> for ConstantArgumentInstruction {
    fn into(self) -> BinaryInstruction {
        match self {
            ConstantArgumentInstruction::BRZ(arg) => BinaryInstruction {
                opcode: 9,
                argument: arg,
            },
            ConstantArgumentInstruction::BRC(arg) => BinaryInstruction {
                opcode: 10,
                argument: arg,
            },
            ConstantArgumentInstruction::BRN(arg) => BinaryInstruction {
                opcode: 11,
                argument: arg,
            },
        }
    }
}

impl<'a> Into<BinaryInstruction> for ArgumentInstruction {
    fn into(self) -> BinaryInstruction {
        match self {
            ArgumentInstruction::LDA(arg) => match arg {
                Argument::MemoryLocation(arg) => BinaryInstruction {
                    opcode: 2,
                    argument: arg,
                },
                Argument::Constant(arg) => BinaryInstruction {
                    opcode: 1,
                    argument: arg,
                },
            },
            ArgumentInstruction::ADD(arg) => match arg {
                Argument::MemoryLocation(arg) => BinaryInstruction {
                    opcode: 5,
                    argument: arg,
                },
                Argument::Constant(arg) => BinaryInstruction {
                    opcode: 4,
                    argument: arg,
                },
            },
            ArgumentInstruction::SUB(arg) => match arg {
                Argument::MemoryLocation(arg) => BinaryInstruction {
                    opcode: 7,
                    argument: arg,
                },
                Argument::Constant(arg) => BinaryInstruction {
                    opcode: 6,
                    argument: arg,
                },
            },
        }
    }
}
