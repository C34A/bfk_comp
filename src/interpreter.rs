use std::io;
use std::io::{Read};

use crate::instructions::{Inst, InstKind};


pub fn interpret(instructions: &Vec<Inst>) {
    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut ptr: usize = 0;
    let mut inst_addr = 0;

    let stdin = io::stdin();
    let mut input = stdin.lock().bytes();

    while inst_addr < instructions.len() {
        let inst = &instructions[inst_addr];
        // println!("inst {} | {:?}", inst_addr, inst);
        match inst.kind {
            InstKind::IncPtr => {
                ptr += inst.times;
                inst_addr += 1;
            },
            InstKind::DecPtr => {
                ptr -= inst.times;
                inst_addr += 1;
            },
            InstKind::IncByte => {
                // memory[ptr] = match memory[ptr] {
                //     255 => 0,
                //     _ => memory[ptr] + 1,
                // };
                memory[ptr] = ((memory[ptr] as usize + inst.times) % 255) as u8;
                inst_addr += 1;
            },
            InstKind::DecByte => {
                // memory[ptr as usize] = match memory[ptr as usize] {
                //     0 => 255,
                //     _ => memory[ptr as usize] - 1,
                // };
                memory[ptr] = wrapping_subtract(memory[ptr], inst.times);
                inst_addr += 1;
            },
            InstKind::WriteByte => {
                for _ in 0..inst.times {
                    print!("{}", memory[ptr as usize] as char);
                }
                inst_addr += 1;
            },
            InstKind::ReadByte => {
                let byte_opt = input.next();
                match byte_opt {
                    Some(Ok(byte)) => {
                        memory[ptr as usize] = byte;
                    },
                    None => {
                        memory[ptr as usize] = 0; // Got EOF -> set to 0
                    },
                    Some(Err(e)) => {
                        panic!("error trying to read byte from input: {}", e);
                    }
                }
            },
            InstKind::LoopStart {end_idx} => {
                if memory[ptr as usize] == 0 {
                    inst_addr = end_idx;
                } else {
                    inst_addr += 1;
                }
            },
            InstKind::LoopEnd {start_idx} => {
                if memory[ptr as usize] != 0 {
                    inst_addr = start_idx;
                } else {
                    inst_addr += 1;
                }
            },
        }
    }
}

fn wrapping_subtract(a: u8, b: usize) -> u8{
    if (a as i32 - b as i32) < 0 {
        wrapping_subtract(a + 255, b)
    } else {
        a - b as u8
    }
}

// for debugging:
// fn dump_memory(start: usize, end: usize, memory: &[u8]) {
//     print!("[{}", memory[start]);
//     for i in start+1..end {
//         print!(", {}", memory[i]);
//     }
//     println!("]");
// }