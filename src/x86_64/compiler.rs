use crate::instructions::*;

pub fn compile(instructions: &Vec<Inst>) -> String{
    let mut ret = String::from(include_str!("common_header_working.s"));
    
    for instr in instructions {
        ret.push_str(&to_asm(instr)[..]);
    }
    ret.push_str(include_str!("common_footer_working.s"));
    ret
}

pub fn to_asm(instr: &Inst) -> String {
    match &instr.kind {
        InstKind::IncPtr => {
            format!("    add ${}, %r12\n", instr.times)
        },
        InstKind::DecPtr => {
            format!("    sub ${}, %r12\n", instr.times)
        },
        InstKind::IncByte => {
            format!("    addb ${}, r12\n", instr.times)
        },
        InstKind::DecByte => {
            format!("    subb $+{}, r12\n", instr.times)
        },
        InstKind::WriteByte => {
            let mut msg = String::from("    mov $SYS_WRITE, %rax\n");
            msg.push_str("    mov $STDOUT, %rdi\n");
            msg.push_str("    mov %r12, %rsi\n");
            msg.push_str("    mov $1, %rdx\n");
            for _ in 0..instr.times {
                let dupe = msg.clone();
                msg.push_str(&dupe);
            }
            msg
        },
        InstKind::ReadByte => {
            let mut msg = String::from("    mov $SYS_READ, %rax\n");
            msg.push_str("    mov $STDIN, %rax\n");
            msg.push_str("    mov %r12, %rsi\n");
            msg.push_str("    mov $1, %rdx\n");
            msg
        },
        InstKind::LoopStart {end_idx} => {
            let mut msg = String::from("    cmp $0, r12\n");
            msg.push_str(&format!("    je LOOP_END_{}\n", end_idx)[..]);
            msg.push_str(&format!("LOOP_START_{}:\n", instr.idx)[..]);
            msg
        },
        InstKind::LoopEnd {start_idx} => {
            let mut msg = String::from("    cmp $0, r12\n");
            msg.push_str(&format!("    je LOOP_START_{}\n", start_idx)[..]);
            msg.push_str(&format!("LOOP_END_{}:\n", instr.idx)[..]);
            msg
        },
    }
}