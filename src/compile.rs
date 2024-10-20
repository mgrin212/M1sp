use std::collections::HashMap;

use nom::combinator::map;

use crate::{
    asm::{
        Directive::{self, *},
        Operand::{self, *},
        Register::*,
    },
    ast::{BinPrim, Expr, UnPrim},
    utils::gensym,
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

fn compile_binary_primitive(stack_index: i64, expr: BinPrim) -> Vec<Directive> {
    match expr {
        BinPrim::Plus => vec![
            Ldr(Reg(X1), stack_address(stack_index)),
            Add(Reg(X0), Reg(X1)),
        ],
        BinPrim::Minus => vec![
            Mov(Reg(X1), Reg(X0)),
            Ldr(Reg(X0), stack_address(stack_index)),
            Sub(Reg(X0), Reg(X1)),
        ],
        BinPrim::Eq => [
            vec![
                Ldr(Reg(X1), stack_address(stack_index)),
                Cmp(Reg(X1), Reg(X0)),
            ],
            zf_to_bool(),
        ]
        .concat(),
        BinPrim::Lt => [
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

fn compile_unary_primitive(expr: UnPrim) -> Vec<Directive> {
    match expr {
        UnPrim::Add1 => vec![Add(Reg(X0), operand_of_num(1))],
        UnPrim::Not => [vec![Cmp(Reg(X0), operand_of_bool(false))], zf_to_bool()].concat(),
        UnPrim::Sub1 => vec![Sub(Reg(X0), operand_of_num(1))],
        UnPrim::IsNum => [
            vec![And(Reg(X0), Imm(NUM_MASK)), Cmp(Reg(X0), Imm(NUM_TAG))],
            zf_to_bool(),
        ]
        .concat(),
        UnPrim::IsZero => [vec![Cmp(Reg(X0), operand_of_num(0))], zf_to_bool()].concat(),
    }
}
pub fn compile_expr(symtab: &HashMap<String, i64>, stack_index: i64, expr: Expr) -> Vec<Directive> {
    match expr {
        Expr::Nil => vec![Mov(Reg(X0), Imm(NIL_TAG))],
        Expr::Num(x) => vec![Mov(Reg(X0), operand_of_num(x))],
        Expr::True => vec![Mov(Reg(X0), operand_of_bool(true))],
        Expr::False => vec![Mov(Reg(X0), operand_of_bool(false))],
        Expr::UnPrim(p1, expr) => [
            compile_expr(symtab, stack_index, *expr),
            compile_unary_primitive(p1),
        ]
        .concat(),
        Expr::BinPrim(f, arg1, arg2) => [
            compile_expr(symtab, stack_index, *arg1),
            vec![Str(stack_address(stack_index), Reg(X0))],
            compile_expr(symtab, stack_index - 8, *arg2),
            compile_binary_primitive(stack_index, f),
        ]
        .concat(),
        Expr::If(test_expr, then_expr, else_expr) => {
            let then_label = gensym("then");
            let else_label = gensym("else");
            let continue_label = gensym("continue");
            [
                compile_expr(symtab, stack_index, *test_expr),
                vec![
                    Cmp(Reg(X0), operand_of_bool(false)),
                    Beq(else_label.clone()),
                    Label(then_label.clone()),
                ],
                compile_expr(symtab, stack_index, *then_expr),
                vec![B(continue_label.clone()), Label(else_label)],
                compile_expr(symtab, stack_index, *else_expr),
                vec![Label(continue_label)],
            ]
            .concat()
        }
        Expr::Id(s) if symtab.contains_key(&s) => {
            vec![Ldr(Reg(X0), stack_address(*symtab.get(&s).unwrap()))]
        }
        Expr::Let(bindings, body) => {
            let mut compiled = Vec::new();
            let mut new_symtab = symtab.clone();

            for (i, (var, exp)) in bindings.iter().enumerate() {
                let mut exp_code =
                    compile_expr(&new_symtab, stack_index - (8 * i as i64), *exp.clone());
                compiled.append(&mut exp_code);
                compiled.push(Str(stack_address(stack_index - (8 * i as i64)), Reg(X0)));
                new_symtab.insert(var.clone(), stack_index - (8 * i as i64));
            }

            let body_code = compile_expr(
                &new_symtab,
                stack_index - (8 * bindings.len() as i64),
                *body,
            );
            compiled.extend(body_code);

            compiled
        }
        Expr::Do(exps) => exps
            .into_iter()
            .flat_map(|e| compile_expr(symtab, stack_index, e))
            .collect(),
        Expr::Define(name, args, body) => {
            let mut new_symtab = symtab.clone();
            let mut directives = vec![
                Label(name.clone()),
                Sub(Reg(Sp), Imm(16)),
                Str(RegOffset(Sp, 8), Reg(Lr)),
                Str(RegOffset(Sp, 0), Reg(Fp)),
                Mov(Reg(Fp), Reg(Sp)),
            ];

            // Store arguments from registers to stack
            for (i, arg) in args.iter().enumerate() {
                directives.push(Str(RegOffset(Fp, 16 + (i as i64 * 8)), Reg(X0 + i as i32)));
                new_symtab.insert(arg.clone(), 16 + (i as i64 * 8));
            }

            let body_code = compile_expr(&new_symtab, -16, *body);
            directives.extend(body_code);
            directives.extend(vec![
                Ldr(Reg(Fp), RegOffset(Sp, 0)),
                Ldr(Reg(Lr), RegOffset(Sp, 8)),
                Add(Reg(Sp), Imm(16)),
                Ret,
            ]);
            directives
        }

        Expr::Call(f, args) => {
            let mut directives = Vec::new();
            let len = args.len();

            // Compile arguments and store them on the stack
            for (i, arg) in args.into_iter().enumerate().rev() {
                let arg_code = compile_expr(symtab, stack_index - (i as i64 * 8), arg);
                directives.extend(arg_code);
                directives.push(Str(RegOffset(Sp, -(8 + (i as i64 * 8))), Reg(X0)));
            }

            // Load arguments from stack into registers
            for i in 0..len.min(8) {
                // Assuming we use up to 8 registers for arguments
                directives.push(Ldr(
                    Reg(X0 + i as i32),
                    RegOffset(Sp, -(8 + ((len - 1 - i) as i64 * 8))),
                ));
            }

            // Adjust stack pointer, call function, and readjust stack pointer
            directives.push(Sub(Reg(Sp), Imm(16 + (len as i64 * 8))));
            directives.push(Bl(f));
            directives.push(Add(Reg(Sp), Imm(16 + (len as i64 * 8))));

            directives
        }
        _ => vec![],
    }
}

pub fn compile(expr: Expr) -> Vec<Directive> {
    let start = vec![
        Global("lisp_entry".to_string()),
        Extern("lisp_error".to_string()),
        Label("lisp_entry".to_string()),
    ];
    let body = compile_expr(&HashMap::new(), -8, expr);
    let ret = vec![Ret];
    [start, body, ret].concat()
}
