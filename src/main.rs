use std::env;
use std::fs;

mod brainfuck;
use brainfuck::BrainFuck;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let src = fs::read_to_string(&args[1]);
    match src {
        Ok(code) => fuck(code),
        Err(err) => eprintln!("{}", err),
    }
}

fn fuck(code: String) {
    let mut fucked = BrainFuck::new(code);
    fucked.start_process();
}

fn usage() -> ! {
    let name: Vec<String> = env::args().collect();
    eprintln!("Usage: {} [source file]", name[0]);
    std::process::exit(1);
}
