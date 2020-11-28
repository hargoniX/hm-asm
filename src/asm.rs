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
