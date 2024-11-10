use std::collections::HashMap;

use crate::{
    asm::{
        Directive::{self, *},
        Operand::{self, *},
        Register::*,
    },
    ast::{BinaryOp, Expr, UnaryOp},
    utils::gensym,
    Definition, Program,
};

pub const NUM_SHIFT: i64 = 2;
pub const NUM_MASK: i64 = 0b11;
pub const NUM_TAG: i64 = 0b00;

pub const BOOL_SHIFT: i64 = 7;
pub const BOOL_MASK: i64 = 0b1111111;
pub const BOOL_TAG: i64 = 0b0011111;

pub const HEAP_MASK: i64 = 0b111;

pub const PAIR_TAG: i64 = 0b010;

pub const NIL_MASK: i64 = 0b11111111;
pub const NIL_TAG: i64 = 0b11111111;

pub const VEC_MASK: i64 = 0b111;
pub const VEC_TAG: i64 = 0b101;
pub fn operand_of_num(x: i64) -> Operand {
    Imm(((x) << NUM_SHIFT) | NUM_TAG)
}

/// Converts a boolean to its runtime representation as an operand for instructions
pub fn operand_of_bool(b: bool) -> Operand {
    Operand::Imm((if b { 1i64 } else { 0i64 } << BOOL_SHIFT) | BOOL_TAG)
}

pub fn zf_to_bool() -> Vec<Directive> {
    vec![
        Mov(Reg(X0), Imm(0)),
        Cset(Reg(X0), "eq".to_string()),
        Lsl(Reg(X0), Imm(BOOL_SHIFT)),
        Orr(Reg(X0), Imm(BOOL_TAG)),
    ]
}

pub fn setl_bool() -> Vec<Directive> {
    vec![
        Mov(Reg(X0), Imm(0)),
        Cset(Reg(X0), "lt".to_string()),
        Lsl(Reg(X0), Imm(BOOL_SHIFT)),
        Orr(Reg(X0), Imm(BOOL_TAG)),
    ]
}

pub fn stack_address(index: i64) -> Operand {
    MemOffset(Box::new(Imm(index)), Box::new(Reg(Sp)))
}

fn calculate_stack_space(count: usize) -> i64 {
    let space = (count as i64 + 1) * 16; // +1 for fp/lr
    (space + 15) & !15 // Round up to nearest multiple of 16
}

fn compile_binary_primitive(stack_index: i64, expr: BinaryOp) -> Vec<Directive> {
    match expr {
        BinaryOp::Add => vec![
            Ldr(Reg(X1), stack_address(stack_index)),
            Add(Reg(X0), Reg(X1)),
        ],
        BinaryOp::Sub => vec![
            Mov(Reg(X1), Reg(X0)),
            Ldr(Reg(X0), stack_address(stack_index)),
            Sub(Reg(X0), Reg(X1)),
        ],
        BinaryOp::Eq => [
            vec![
                Ldr(Reg(X1), stack_address(stack_index)),
                Cmp(Reg(X1), Reg(X0)),
            ],
            zf_to_bool(),
        ]
        .concat(),
        BinaryOp::Lt => [
            vec![
                Ldr(Reg(X1), stack_address(stack_index)),
                Cmp(Reg(X1), Reg(X0)),
            ],
            setl_bool(),
        ]
        .concat(),
        _ => vec![],
    }
}

fn compile_unary_primitive(expr: UnaryOp) -> Vec<Directive> {
    match expr {
        UnaryOp::Add1 => vec![Add(Reg(X0), operand_of_num(1))],
        UnaryOp::Not => [vec![Cmp(Reg(X0), operand_of_bool(false))], zf_to_bool()].concat(),
        UnaryOp::Sub1 => vec![Sub(Reg(X0), operand_of_num(1))],
        UnaryOp::IsNum => [
            vec![And(Reg(X0), Imm(NUM_MASK)), Cmp(Reg(X0), Imm(NUM_TAG))],
            zf_to_bool(),
        ]
        .concat(),
        UnaryOp::IsZero => [vec![Cmp(Reg(X0), operand_of_num(0))], zf_to_bool()].concat(),
        _ => vec![],
    }
}
pub fn compile_expr(
    definitions: &Vec<Definition>,
    symtab: &HashMap<String, i64>,
    stack_index: i64,
    expr: Expr,
) -> Vec<Directive> {
    dbg!(expr.clone(), stack_index);
    match expr {
        Expr::Unit => vec![Mov(Reg(X0), Imm(NIL_TAG))],
        Expr::Num(x) => vec![Mov(Reg(X0), operand_of_num(x))],
        Expr::Bool(b) => vec![Mov(Reg(X0), operand_of_bool(b))],
        Expr::UnOp(p1, expr) => [
            compile_expr(definitions, symtab, stack_index, *expr),
            compile_unary_primitive(p1),
        ]
        .concat(),
        Expr::BinOp(f, arg1, arg2) => {
            let aligned_index = (stack_index - 15) & !15;
            [
                compile_expr(definitions, symtab, aligned_index, *arg1),
                vec![Str(stack_address(aligned_index), Reg(X0))],
                compile_expr(definitions, symtab, aligned_index - 16, *arg2),
                compile_binary_primitive(aligned_index, f),
            ]
            .concat()
        }
        Expr::If(test_expr, then_expr, else_expr) => {
            let then_label = gensym("then");
            let else_label = gensym("else");
            let continue_label = gensym("continue");
            [
                compile_expr(definitions, symtab, stack_index, *test_expr),
                vec![
                    Cmp(Reg(X0), operand_of_bool(false)),
                    Beq(else_label.clone()),
                    Label(then_label.clone()),
                ],
                compile_expr(definitions, symtab, stack_index, *then_expr),
                vec![B(continue_label.clone()), Label(else_label)],
                compile_expr(definitions, symtab, stack_index, *else_expr),
                vec![Label(continue_label)],
            ]
            .concat()
        }
        Expr::Id(s) if symtab.contains_key(&s) => {
            vec![Ldr(Reg(X0), stack_address(*symtab.get(&s).unwrap()))]
        }
        Expr::Let(bindings, body) => {
            let stack_space = calculate_stack_space(bindings.len());

            let compiled: Vec<Directive> = bindings
                .clone()
                .into_iter()
                .enumerate()
                .flat_map(|(i, (_, e))| {
                    let offset = i as i64 * 16;
                    dbg!(e.clone(), stack_index - offset);
                    [
                        compile_expr(definitions, symtab, stack_index - offset, *e),
                        vec![Str(stack_address(stack_index - offset), Reg(X0))],
                    ]
                    .concat()
                })
                .collect();
            let new_symtab = bindings
                .clone()
                .into_iter()
                .enumerate()
                .map(|(i, (v, _))| (v, stack_index - (i as i64 * 16)))
                .collect::<HashMap<String, i64>>();
            [
                vec![Sub(Reg(Sp), Imm(stack_space))],
                compiled,
                compile_expr(
                    definitions,
                    &new_symtab,
                    stack_index - (bindings.len() as i64 * 16),
                    *body,
                ),
                vec![Add(Reg(Sp), Imm(stack_space))],
            ]
            .concat()
        }
        Expr::Do(exps) => exps
            .into_iter()
            .flat_map(|e| compile_expr(definitions, symtab, stack_index, *e))
            .collect(),
        Expr::Call(name, args) => {
            let compiled_args: Vec<Directive> = args
                .clone()
                .into_iter()
                .enumerate()
                .flat_map(|(i, arg)| {
                    [
                        compile_expr(definitions, symtab, stack_index - (i as i64 * 16), *arg),
                        vec![Str(stack_address(stack_index - (i as i64 * 16)), Reg(X0))],
                    ]
                    .concat()
                })
                .collect();
            [
                vec![Sub(Reg(Sp), Imm(64))],
                vec![Raw("stp fp, lr, [sp]".to_string())],
                compiled_args,
                vec![
                    Bl(name),
                    Raw("ldp fp, lr, [sp]".to_string()),
                    Add(Reg(Sp), Imm(64)),
                ],
            ]
            .concat()
        }
        _ => vec![],
    }
}

fn align_stack_index(index: i64) -> i64 {
    if index % 16 == -8 {
        index
    } else {
        index - 8
    }
}

fn get_defn(defns: &[Definition], name: String) -> Option<&Definition> {
    defns.iter().find(|d| match d {
        Definition(def_name, _, _) => *def_name == name,
    })
}

pub fn compile_definitions(
    defs: &Vec<Definition>,
    Definition(name, args, body): &Definition,
) -> Vec<Directive> {
    let stack_space = 48;
    let ftab: HashMap<String, i64> = args
        .into_iter()
        .enumerate()
        .map(|(i, arg)| (arg.clone(), (i as i64 + 2) * -16)) // Offset by 64 to account for caller's frame
        .collect();

    [
        vec![
            Label(name.clone()),
            Sub(Reg(Sp), Imm(stack_space)),
            Raw("stp fp, lr, [sp]".to_string()),
            Mov(Reg(Fp), Reg(Sp)), // Set frame pointer
        ],
        compile_expr(defs, &ftab, (args.len() as i64 + 2) * -16, body.clone()),
        vec![
            Raw("ldp fp, lr, [sp]".to_string()),
            Add(Reg(Sp), Imm(stack_space)),
            Ret,
        ],
    ]
    .concat()
}

pub fn compile(Program(defs, expr): Program) -> Vec<Directive> {
    let start = vec![
        Global("lisp_entry".to_string()),
        Extern("lisp_error".to_string()),
        Label("lisp_entry".to_string()),
    ];
    let body = compile_expr(&defs, &HashMap::new(), -16, expr);
    let definitions: Vec<Directive> = defs
        .clone()
        .into_iter()
        .flat_map(|d| compile_definitions(&defs.clone(), &d.clone()))
        .collect();
    let ret = vec![Ret];
    [start, body, ret, definitions].concat()
}
