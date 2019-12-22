use std::env;
use std::fs;

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
    let mut buf: [u8; 2048] = [0; 2048];
    let mut b_index: usize = 0;
    let code = code.chars();
    let max = code.clone().count();
    let mut c_index: usize = 0;
    loop {
        if c_index == max {
            std::process::exit(0);
        }
        let cur = code.clone().nth(c_index).unwrap();
        match cur {
            '+' => {
                buf[b_index] += 1;
                c_index += 1;
            },
            '-' => {
                buf[b_index] -= 1;
                c_index += 1;
            },
            '>' => {
                b_index += 1;
                c_index += 1;
            },
            '<' => {
                b_index -= 1;
                c_index += 1;
            },
            '.' => {
                print!("{}", buf[b_index] as char);
                c_index += 1;
            },
            ',' => {
                getc(&mut buf[b_index]);
                c_index += 1;
            },
            '[' => {
                if buf[b_index] == 0 {
                    let mut nest: i64 = 0;
                    let mut _cur: char = 0 as char;
                    loop {
                        c_index += 1;
                        _cur = code.clone().nth(c_index).unwrap();
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
                c_index += 1;
            },
            ']' => {
                if buf[b_index] != 0 {
                    let mut nest: i64 = 0;
                    let mut _cur: char = 0 as char;
                    loop {
                        c_index -= 1;
                        _cur = code.clone().nth(c_index).unwrap();
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
                c_index += 1;
            },
            tmp if tmp == _eof => break,
            _ => c_index += 1,
        }
    }
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
