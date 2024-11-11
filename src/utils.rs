use std::sync::atomic::AtomicUsize;

use crate::asm::string_of_directive;
use crate::asm::Directive;
use crate::asm::{Directive::*, Operand::*, Register::*};

macro_rules! combine_directives {
    ($($directive:expr),+ $(,)?) => {{
        let combined = vec![$(string_of_directive(&$directive)),+]
            .into_iter()
            .collect::<Vec<String>>()
            .join("\n");
        Directive::R_(combined)
    }};
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
pub fn gensym(prefix: &str) -> String {
    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{}_{}", prefix, count)
}

pub fn create_stack_frame(size: i64) -> Directive {
    let stack_frame_size = 16 + size;
    combine_directives!(
        Sub(Reg(Sp), Imm(stack_frame_size)),
        Stp(Reg(Fp), Reg(Lr), RegOffset(Sp, 16)),
        Mov(Reg(Fp), Reg(Sp))
    )
}

pub fn destroy_stack_frame(size: i64) -> Directive {
    let stack_frame_size = 16 + size;
    combine_directives!(
        Ldp(Reg(Fp), Reg(Lr), RegOffset(Sp, 16)),
        Add(Reg(Sp), Imm(stack_frame_size))
    )
}
