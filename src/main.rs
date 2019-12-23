use std::env;
use std::fs;

struct BrainFuck<'a> {
    code: &'a std::str::Chars<'a>,
    c_index: usize,
    buf: Vec<u8>,
    b_index: usize
}


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
    let _eof: char = 0 as char;
    let mut fucked = BrainFuck {
        code: &code.chars(),
        c_index: 0,
        buf: Vec::new(),
        b_index: 0
    };
    fucked.buf.push(0);
    let max = code.chars().count();

    loop {
        if fucked.c_index == max {
            std::process::exit(0);
        }
        let cur = fucked.code.clone().nth(fucked.c_index).unwrap();
        match cur {
            '+' => {
                fucked.buf[fucked.b_index] += 1;
                fucked.c_index += 1;
            },
            '-' => {
                fucked.buf[fucked.b_index] -= 1;
                fucked.c_index += 1;
            },
            '>' => {
                fucked.buf.push(0);
                fucked.b_index += 1;
                fucked.c_index += 1;
            },
            '<' => {
                fucked.b_index -= 1;
                fucked.c_index += 1;
            },
            '.' => {
                let c: &u8 = fucked.buf.get(fucked.b_index).unwrap_or(&0);
                print!("{}", *c as char);
                fucked.c_index += 1;
            },
            ',' => {
                getc(&mut fucked.buf[fucked.b_index]);
                fucked.c_index += 1;
            },
            '[' => bf_loop_start(&mut fucked),
            ']' => bf_loop_gool(&mut fucked),
            tmp if tmp == _eof => break,
            _ => fucked.c_index += 1,
        }
    }
}

fn bf_loop_start(fucked: &mut BrainFuck) {
    if fucked.buf[fucked.b_index] == 0 {
        let mut nest: i64 = 0;
        let mut _cur: char = 0 as char;
        loop {
            fucked.c_index += 1;
            _cur = fucked.code.clone().nth(fucked.c_index).unwrap();
            if _cur == '[' {
                nest += 1;
            } else if _cur == ']' {
                nest -= 1;
                if nest < 0 {
                    break;
                }
            }
        }
    }
    fucked.c_index += 1;
}

fn bf_loop_gool(fucked: &mut BrainFuck) {
    if fucked.buf[fucked.b_index] != 0 {
        let mut nest: i64 = 0;
        let mut _cur: char = 0 as char;
        loop {
            fucked.c_index -= 1;
            _cur = fucked.code.clone().nth(fucked.c_index).unwrap();
            if _cur == ']' {
                nest += 1;
            } else if _cur =='[' {
                nest -= 1;
                if nest < 0 {
                    break;
                }
            }
        }
    }
    fucked.c_index += 1;
}

fn usage() {
    let name: Vec<String> = env::args().collect();
    eprintln!("Usage: {} [source file]", name[0]);
    std::process::exit(1);
}

fn getc(buf: &mut u8) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .ok().expect("Failed to read line.");
    *buf = input.bytes().nth(0)
        .unwrap_or('\n' as u8);
}