use std::{fmt, ops};

pub fn test() {
    println!("From another File");
}

#[derive(Clone, Debug)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    Sp,
    Lr,
    Fp,
}

#[derive(Clone, Debug)]
pub enum Operand {
    Reg(Register),
    Imm(i64),
    MemOffset(Box<Operand>, Box<Operand>),
    RegOffset(Register, i64)
}

#[derive(Clone, Debug)]
pub enum Directive {
    Global(String),
    Extern(String),
    Section(String),
    Label(String),
    DqLabel(String),
    DqString(String),
    DqInt(i64),
    Align(i32),
    Mov(Operand, Operand),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Sdiv(Operand, Operand),
    Lsl(Operand, Operand),
    Lsr(Operand, Operand),
    Asr(Operand, Operand),
    Cmp(Operand, Operand),
    And(Operand, Operand),
    Orr(Operand, Operand),
    Cset(Operand, String),
    Adr(Operand, String),
    B(String),
    Beq(String),
    Bne(String),
    Blt(String),
    Bge(String),
    Bgt(String),
    Ble(String),
    Br(Operand),
    Str(Operand, Operand),
    Ldr(Operand, Operand),
    Stp(Operand, Operand, Operand),
    Ldp(Operand, Operand, Operand),
    Bl(String),
    Ret,
    Comment(String),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Reg(r) => write!(f, "{}", string_of_register(r)),
            Operand::Imm(i) => write!(f, "#{}", i),
            Operand::MemOffset(offset, base) => write!(f, "[{}, {}]", base, offset),
            Operand::RegOffset(reg, offset) => {
                if *offset >= 0 {
                    write!(f, "[{}, #{}]", string_of_register(reg), offset)
                } else {
                    write!(f, "[{}, #-{}]", string_of_register(reg), offset.abs())
                }
            }
        }
    }
}

pub fn string_of_register(reg: &Register) -> String {
    match reg {
        Register::X0 => String::from("X0"),
        Register::X1 => String::from("X1"),
        Register::X2 => String::from("X2"),
        Register::X3 => String::from("X3"),
        Register::X4 => String::from("X4"),
        Register::X5 => String::from("X5"),
        Register::X6 => String::from("X6"),
        Register::X7 => String::from("X7"),
        Register::X8 => String::from("X8"),
        Register::X9 => String::from("X9"),
        Register::X10 => String::from("X10"),
        Register::X11 => String::from("X11"),
        Register::X12 => String::from("X12"),
        Register::X13 => String::from("X13"),
        Register::X14 => String::from("X14"),
        Register::X15 => String::from("X15"),
        Register::X16 => String::from("X16"),
        Register::Sp => String::from("sp"),
        Register::Lr => String::from("lr"),
        Register::Fp => String::from("fp"),
    }
}

impl ops::Add<i32> for Register {
    type Output = Register;

    fn add(self, rhs: i32) -> Self::Output {
        match rhs {
            1 => Self::X1,
            2 => Self::X2,
            3 => Self::X3,
            4 => Self::X4,
            5 => Self::X5,
            6 => Self::X6,
            7 => Self::X7,
            8 => Self::X8,
            9 => Self::X9,
            10 => Self::X10,
            11 => Self::X11,
            12 => Self::X12,
            13 => Self::X13,
            14 => Self::X14,
            15 => Self::X15,
            16 => Self::X16,
            _ => Self::X0,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::X0 => write!(f, "X0"),
            Register::X1 => write!(f, "X1"),
            Register::X2 => write!(f, "X2"),
            Register::X3 => write!(f, "X3"),
            Register::X4 => write!(f, "X4"),
            Register::X5 => write!(f, "X5"),
            Register::X6 => write!(f, "X7"),
            Register::X7 => write!(f, "X7"),
            Register::X8 => write!(f, "X8"),
            Register::X9 => write!(f, "X9"),
            Register::X10 => write!(f, "X10"),
            Register::X11 => write!(f, "X11"),
            Register::X12 => write!(f, "X12"),
            Register::X13 => write!(f, "X13"),
            Register::X14 => write!(f, "X14"),
            Register::X15 => write!(f, "X15"),
            Register::X16 => write!(f, "X16"),
            Register::Sp => write!(f, "sp"),
            Register::Lr => write!(f, "lr"),
            Register::Fp => write!(f, "fp"),
        }
    }
}

pub fn is_register(o: Operand) -> bool {
    match o {
        Operand::Reg(_) => true,
        _ => false,
    }
}

pub fn label_name(label: &str) -> String {
    format!("_{}", label)
}

pub fn string_of_directive(directive: &Directive) -> String {
    match directive {
        Directive::Global(l) => format!(".global _{}", l),
        Directive::Extern(l) => format!(".extern {}", label_name(l)),
        Directive::Section(l) => format!(".section {}", l),
        Directive::Label(l) => format!("{}:", label_name(l)),
        Directive::DqLabel(l) => format!(".quad {}", label_name(l)),
        Directive::DqString(l) => format!(".ascii \"{}\"\n\t.byte 0", l.escape_default()),
        Directive::DqInt(i) => format!(".quad {}", i),
        Directive::Align(i) => format!(".align {}", i),
        Directive::Mov(dest, src) => format!("\tmov {}, {}", dest, src),
        Directive::Add(dest, src) => format!("\tadd {}, {}, {}", dest, dest, src),
        Directive::Sub(dest, src) => format!("\tsub {}, {}, {}", dest, dest, src),
        Directive::Mul(dest, src) => format!("\tmul {}, {}, {}", dest, dest, src),
        Directive::Sdiv(dest, src) => format!("\tsdiv {}, {}, {}", dest, dest, src),
        Directive::Lsl(dest, src) => format!("\tlsl {}, {}, {}", dest, dest, src),
        Directive::Lsr(dest, src) => format!("\tlsr {}, {}, {}", dest, dest, src),
        Directive::Asr(dest, src) => format!("\tasr {}, {}, {}", dest, dest, src),
        Directive::Cmp(dest, src) => format!("\tcmp {}, {}", dest, src),
        Directive::And(dest, src) => format!("\tand {}, {}, {}", dest, dest, src),
        Directive::Orr(dest, src) => format!("\torr {}, {}, {}", dest, dest, src),
        Directive::Cset(dest, cond) => format!("\tcset {}, {}", dest, cond),
        Directive::Adr(dest, label) => format!("\tadr {}, {}", dest, label_name(label)),
        Directive::B(dest) => format!("\tb {}", label_name(dest)),
        Directive::Beq(dest) => format!("\tbeq {}", label_name(dest)),
        Directive::Bne(dest) => format!("\tbne {}", label_name(dest)),
        Directive::Blt(dest) => format!("\tblt {}", label_name(dest)),
        Directive::Bge(dest) => format!("\tbge {}", label_name(dest)),
        Directive::Bgt(dest) => format!("\tbgt {}", label_name(dest)),
        Directive::Ble(dest) => format!("\tble {}", label_name(dest)),
        Directive::Br(dest) => format!("\tbr {}", dest),
        Directive::Str(dest, src) => format!("\tstr {}, {}", src, dest),
        Directive::Ldr(dest, src) => format!("\tldr {}, {}", dest, src),
        Directive::Stp(src1, src2, dest) => format!("\tstp {}, {}, {}", src1, src2, dest),
        Directive::Ldp(dest1, dest2, src) => format!("\tldp {}, {}, {}", dest1, dest2, src),
        Directive::Bl(dest) => format!("\tbl {}", label_name(dest)),
        Directive::Ret => "\tret".to_string(),
        Directive::Comment(s) => format!("// {}", s),
    }
}
