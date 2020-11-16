use std::env;
use std::fs;

use bfk_comp_one::instructions::Inst;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough args: {:?}", args);
    }
    let mut debug = false;
    if args.len() > 2 {
        if args[2] == "-D" {
            debug = true;
        }
    }
    
    let contents = fs::read_to_string(&args[1]).expect("wasn't able to read file");
    let instructions: Vec<Inst> = bfk_comp_one::lex(contents);

    if debug {
        println!("{}", bfk_comp_one::dump_instructions(&instructions));
    }
    println!("interpreting...");
    bfk_comp_one::interpreter::interpret(instructions);
}
