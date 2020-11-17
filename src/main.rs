use std::env;
use std::fs;

use bfk_comp_one::instructions::Inst;

enum Mode {
    Compile,
    Interpret,
    Unspecified,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough args: {:?}", args);
    }
    let mut mode = Mode::Unspecified;
    let mut debug = false;
    let mut outfile: Option<&str> = None;
    if args.len() > 2 {
        mode = match &args[1].to_ascii_lowercase()[..] {
            "compile" => Mode::Compile,
            "interpret" => Mode::Interpret,
            msg => {
                panic!("unkown mode: {:?}", msg);
            }
        }
    }
    for (idx, arg) in args.iter().enumerate() {
        let slice = &arg[..];
        match slice {
            "-Debug" => {
                debug = true;
            },
            "-o" => {
                // outfile = Some(fs::File::create(
                //     args.get(idx + 1).expect("please provide filename with -o!")
                // ).expect("Failed to open file!"));
                outfile = Some(
                    args.get(idx + 1).expect("please give a filename with -o!")
                );
            }
            _ => (),
        }
    }



    
    let contents = fs::read_to_string(&args[2]).expect("wasn't able to read file");
    let instructions: Vec<Inst> = bfk_comp_one::lex(contents);

    if debug {
        println!("{}", bfk_comp_one::dump_instructions(&instructions));
    }
    match mode {
        Mode::Compile => {
            println!("Compiling...\n");
            let code = bfk_comp_one::x86_64::compiler::compile(&instructions);
            match outfile {
                None => {
                    println!("{}", code);
                },
                Some(file) => {
                    // std::boxed::Box::new(file).write_all(code);
                    let ret = fs::write(file, code);
                    match ret {
                        Err(e) => {
                            panic!("failed to write file {}: {}", file, e);
                        },
                        _ => (),
                    }
                },
            }
        },
        Mode::Interpret => {
            println!("interpreting...");
            bfk_comp_one::interpreter::interpret(&instructions);
        }
        _ => {
            println!("mode unspecified. instructions parsed:\n{}", 
            bfk_comp_one::dump_instructions(&instructions));
        }
    }
    
    // println!("interpreting...");
    // bfk_comp_one::interpreter::interpret(instructions);
}
