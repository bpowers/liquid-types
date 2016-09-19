use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error};
use std::vec::Vec;

pub type Register = i32;
pub type Label = String;

pub struct Machine {
    pub regs: RegFile,
    pub heap: Heap,
    pub iseq: Vec<Instr>,
}

pub struct Heap {
    pub blocks: HashMap<Label, Block>,
}

pub struct RegFile {
    // linear mapping from Register -> Value
    pub reg: Vec<Value>,
}

pub struct Program {
    pub blocks: Vec<Block>,
}

pub enum Instr {
    Move(Register, Value),
    Add(Register, Register, Value),
    Cond(Register, Value),
}

pub struct Block {
    pub name: Label,
    pub instrs: Vec<Instr>,
    pub target: Value,
}

pub enum Value {
    VInt(i32),
    VLabel(Label),
    VReg(Register),
}


impl Debug for Program {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        for block in &self.blocks {
            write!(fmt, "{:?}\n", block).unwrap()
        }
        Ok(())
    }
}

impl Debug for Block {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}:\t", self.name).unwrap();

        for instr in &self.instrs {
            write!(fmt, "{:?};\n\t", instr).unwrap()
        }

        write!(fmt, "jump {:?}\n", self.target)
    }
}

impl Debug for Value {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Value::*;
        match *self {
            VInt(n) => write!(fmt, "{}", n),
            VLabel(ref l) => write!(fmt, "{}", l),
            VReg(ref r) => write!(fmt, "r{}", r),
        }
    }
}

impl Debug for Instr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Instr::*;
        match *self {
            Move(r, ref op) => write!(fmt, "r{:?} := {:?}", r, op),
            Add(r1, r2, ref op) => write!(fmt, "r{:?} := r{:?} + {:?}", r1, r2, op),
            Cond(r, ref op) => write!(fmt, "if r{:?} jump {:?}", r, op),
        }
    }
}
