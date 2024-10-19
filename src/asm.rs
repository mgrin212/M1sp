use std::fmt;

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
    X16,
    Sp,
}

#[derive(Clone, Debug)]
pub enum Operand {
    Reg(Register),
    Imm(i64),
    MemOffset(Box<Operand>, Box<Operand>),
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
            Operand::Imm(i) => write!(f, "#{}", i.to_string()),
            Operand::MemOffset(offset, base) => {
                write!(f, "[{}, {}]", base, offset)
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
        Register::X16 => String::from("X16"),
        Register::Sp => String::from("sp"),
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
            Register::X16 => write!(f, "X16"),
            Register::Sp => write!(f, "sp"),
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
