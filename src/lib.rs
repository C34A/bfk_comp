pub mod instructions;
pub mod interpreter;

use instructions::{Inst, InstKind};

pub fn lex(code: String) -> Vec<Inst>{
    let mut instructions: Vec<Inst> = vec![];
    // Keep track of LoopStarts that don't have and end yet
    let mut needs_end: Vec<usize> = vec![]; 

    for (i,c) in code.chars().enumerate() {
        let this_inst = instructions.len();
        let this_inst_kind: Option<InstKind> = match c {
            '>' => Some(InstKind::IncPtr),
            '<' => Some(InstKind::DecPtr),
            '+' => Some(InstKind::IncByte),
            '-' => Some(InstKind::DecByte),
            '.' => Some(InstKind::WriteByte),
            ',' => Some(InstKind::ReadByte),
            '[' => {
                let i = InstKind::LoopStart { end_idx: 0 };
                needs_end.push(this_inst);
                Some(i)
            },
            ']' => {
                let last_idx: usize = needs_end.pop().expect(&format!("imballance of []s! index: {}", i)[..]);
                match instructions.get_mut(last_idx) {
                    Some(t) => {
                    match &mut t.kind {
                        InstKind::LoopStart{ref mut end_idx} => {
                        let idx = this_inst;
                        *end_idx = idx;
                        },
                        _ => {
                        panic!("mixup in loop start/end tracking!\nlast idx: {}\n{}", 
                            last_idx, dump_instructions(&instructions));
                        }
                    }
                    },
                    _ => (),
                };
                Some(InstKind::LoopEnd { 
                    start_idx: last_idx
                })
            },
            _ => None,
        };

        match this_inst_kind {
            Some(instr) => {
                let mut inc_times = false;
                if instructions.len() > 0 {
                    let last_idx = instructions.len()-1;
                    let mut last_instr = &mut instructions[last_idx];
                    // if last_instr.kind == instr {
                    //     last_instr.times += 1;
                    // } else {
                    //     instructions.push(Inst {
                    //         idx: instructions.len(),
                    //         kind: instr,
                    //         times: 1,
                    //     });
                    // }
                    inc_times = match instr {
                        InstKind::LoopStart {end_idx: _} => false,
                        InstKind::LoopEnd {start_idx: _} => false,
                        _ => {
                            if last_instr.kind == instr {
                                last_instr.times += 1;
                                true
                            } else {
                                false
                            }
                        },

                    };
                }
                if !inc_times {
                    instructions.push(Inst {
                        idx: instructions.len(),
                        kind: instr,
                        times: 1,
                    });
                }
                
            },
            _ => (),
        };
    }

    instructions
}

pub fn dump_instructions(instrs: &Vec<Inst>) -> String {
    let mut ret = String::new();
    ret.push_str("instructions:\n");
    for (index, inst) in instrs.iter().enumerate() {
        // println!("{} | {:?}", index, inst);
        ret.push_str(&format!("{} | {:?}\n", index, inst)[..]);
    }
    ret
}