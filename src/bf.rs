extern crate libc;
extern crate colored;

use colored::*;
use std::fs;

pub struct Flg {
    pub dump: bool
}

struct BrainFuck {
    code: String,
    c_index: usize,
    buf: Vec<u8>,
    b_index: usize
}

impl BrainFuck {
    fn new(code: String) -> Self {
        let mut ret = Self {
                code: code,
                c_index: 0,
                buf: Vec::new(),
                b_index: 0
        };
        ret.buf.push(0);
        return ret;
    }

    fn offset_reached_length(&mut self) -> bool {
        self.code.chars().count() == self.c_index
    }

    fn getcur(&self) -> char {
        self.code.clone().chars().nth(self.c_index).unwrap()   
    }

    fn loop_start(&mut self) {
        if self.buf[self.b_index] == 0 {
            let mut nest: i64 = 0;
            let mut _cur: char = 0 as char;
            loop {
                self.c_index += 1;
                _cur = self.getcur();
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
        self.c_index += 1;
    }

    fn loop_gool(&mut self) {
        if self.buf[self.b_index] != 0 {
            let mut nest: i64 = 0;
            let mut _cur: char = 0 as char;
            loop {
                self.c_index -= 1;
                _cur = self.getcur();
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
        self.c_index += 1;
    }

    fn b_inc(&mut self) {
        self.buf[self.b_index] += 1;
        self.c_index += 1;
    }

    fn b_dec(&mut self) {
        self.buf[self.b_index] -= 1;
        self.c_index += 1;
    }

    fn p_inc(&mut self) {
        if self.buf[self.b_index] == *self.buf.last().unwrap() {
            self.buf.push(0);
        }
        self.b_index += 1;
        self.c_index += 1;

    }

    fn p_dec(&mut self) {
        self.b_index -= 1;
        self.c_index += 1;
    
    }

    fn cur_print(&mut self) {
        let c: &u8 = self.buf.get(self.b_index).unwrap_or(&0);
        print!("{}", *c as char);
        self.c_index += 1;
    }

    fn getchar(&mut self) {
        unsafe {
            self.buf[self.b_index] = libc::getchar() as u8;
        }
        self.c_index += 1;
    }

    fn dump(&self) {
        println!();
        println!("{}", "-8<-----8<----8<-----".cyan());
        println!("-----program end-----");
        println!("{}", "let's dump buffer↓ and last pointer's index is red.".red());

        print!("+-----+");
        let len = self.buf.len();
        for _ in 0..len {
            print!("------+");
        }
        println!();
    
        print!("|index|");
        for i in 0..len {
            let index = format!("0x{:>04x}", i);
            if i == self.b_index {
                print!("{}", index.red());
            } else {
                print!("{}", index);
            }
            print!("|");
        }
        println!();
    
        print!("+-----+");
        for _ in 0..len {
            print!("------+");
        }
        println!();
    
        print!("|value|");
        for c in self.buf.clone() {
            print!("0x{:>04x}|", c);
        }
        println!();
    
        print!("+-----+");
        for _ in 0..len {
            print!("------+");
        }
        println!();
    }
}

pub fn machine(path: String, flg: Flg) {
    let file = fs::read_to_string(path);

    match file {
        Ok(path) => fuck(path, flg),
        Err(err) => eprintln!("{}", err)
    }
}

fn fuck(code: String, flg: Flg) {
    if flg.dump {
        println!("dump ok");
    }
    let _eof: char = 0 as char;
    let mut fucked = BrainFuck::new(code);

    loop {
        if fucked.offset_reached_length() {
            if flg.dump  {
                fucked.dump();
            }
            break;
        }
        match fucked.getcur() {
            '+' => fucked.b_inc(),
            '-' => fucked.b_dec(),
            '>' => fucked.p_inc(),
            '<' => fucked.p_dec(),
            '.' => fucked.cur_print(),
            ',' => fucked.getchar(),
            '[' => fucked.loop_start(),
            ']' => fucked.loop_gool(),
            tmp if tmp == _eof => break,
            _ => fucked.c_index += 1,
        }
    }
}
