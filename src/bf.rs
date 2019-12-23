struct BrainFuck {
    code: String,
    c_index: usize,
    buf: Vec<u8>,
    b_index: usize
}

impl BrainFuck {
    fn new(code: String) -> BrainFuck {
        let ret = BrainFuck{
                code: code,
                c_index: 0,
                buf: Vec::new(),
                b_index: 0
        }; 
        return ret;
    }
    fn dump(&self) {
        println!("-8<-----8<----8<-----");
        println!("program end");
        println!("dump buffer");
        print!("+-----+");
        let len = self.buf.len();
        for _ in 0..len {
            print!("----+");
        }
        println!("");
    
        print!("index |");
        for i in 0..len {
            print!("{:>04}|", i);
        }
        println!("");
    
        print!("+-----+");
        for _ in 0..len {
            print!("----+");
        }
        println!("");
    
        print!("val   |");
        for cur in self.buf.clone() {
            print!("{:>04}|", cur);
        }
        println!("");
    
        print!("+-----+");
        for _ in 0..len {
            print!("----+");
        }
        println!("");
    }

}

pub fn fuck(code: String) {
    let _eof: char = 0 as char;
    let mut fucked = BrainFuck::new(code.clone());
    fucked.buf.push(0);
    let max = code.chars().count();

    loop {
        if fucked.c_index == max {
            fucked.dump();
            std::process::exit(0);
        }
        let cur = fucked.code.clone().chars().nth(fucked.c_index).unwrap();
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
                if fucked.buf[fucked.b_index] == *fucked.buf.last().unwrap() {
                    fucked.buf.push(0);
                }
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
            _cur = fucked.code.clone().chars().nth(fucked.c_index).unwrap();
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
            _cur = fucked.code.clone().chars().nth(fucked.c_index).unwrap();
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

fn getc(buf: &mut u8) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .ok().expect("Failed to read line.");
    *buf = input.bytes().nth(0)
        .unwrap_or('\n' as u8);
}
