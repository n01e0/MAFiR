pub struct BrainFuck {
    pub code: Vec<u8>,
    pub c_index: usize,
    pub buf: Vec<u8>,
    pub b_index: usize,
}

impl BrainFuck {
    pub fn new(code: String) -> Self {
        Self {
            code: code.as_bytes().to_vec(),
            c_index: 0,
            buf: Vec::new(),
            b_index: 0,
        }
    }
    pub fn start_process(&mut self) {
        self.buf.push(0);
        loop {
            if self.offset_reached_length() {
                self.dump();
                std::process::exit(0);
            }
            if self.current_is_eof() {
                break;
            }
            let cur = self.looking_byte() as char;
            match cur {
                '+' => {
                    self.buf[self.b_index] += 1;
                    self.c_index += 1;
                }
                '-' => {
                    self.buf[self.b_index] -= 1;
                    self.c_index += 1;
                }
                '>' => {
                    if self.buf[self.b_index] == *self.buf.last().unwrap() {
                        self.buf.push(0);
                    }
                    self.b_index += 1;
                    self.c_index += 1;
                }
                '<' => {
                    self.b_index -= 1;
                    self.c_index += 1;
                }
                '.' => {
                    let c: &u8 = self.buf.get(self.b_index).unwrap_or(&0);
                    print!("{}", *c as char);
                    self.c_index += 1;
                }
                ',' => {
                    self.get_byte_from_stdin();
                    self.c_index += 1;
                }
                '[' => self.loop_start(),
                ']' => self.loop_gool(),
                _ => self.c_index += 1,
            }
        }
    }
    pub fn dump(&self) {
        println!("-8<-----8<----8<-----");
        println!("program end");
        println!("dump buffer");
        print!("+-----+");
        for _ in 0..self.buf.len() {
            print!("----+");
        }
        println!("");

        print!("index |");
        for i in 0..self.buf.len() {
            print!("{:>04}|", i);
        }
        println!("");

        print!("+-----+");
        for _ in 0..self.buf.len() {
            print!("----+");
        }
        println!("");

        print!("val   |");
        for cur in self.buf.clone() {
            print!("{:>04}|", cur);
        }
        println!("");

        print!("+-----+");
        for _ in 0..self.buf.len() {
            print!("----+");
        }
        println!("");
    }
    fn loop_start(&mut self) {
        if self.buf[self.b_index] == 0 {
            let mut nest: i64 = 0;
            let mut _cur: char = 0 as char;
            loop {
                self.c_index += 1;
                _cur = self.code[self.c_index] as char;
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
                _cur = self.code[self.c_index] as char;
                if _cur == ']' {
                    nest += 1;
                } else if _cur == '[' {
                    nest -= 1;
                    if nest < 0 {
                        break;
                    }
                }
            }
        }
        self.c_index += 1;
    }
    fn current_is_eof(&mut self) -> bool {
        self.looking_byte() == 0x00
    }
    fn looking_byte(&mut self) -> u8 {
        self.code[self.c_index]
    }
    fn offset_reached_length(&mut self) -> bool {
        self.c_index == self.code.len()
    }
    fn get_byte_from_stdin(&mut self) {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line.");
        self.buf[self.b_index] = input.bytes().nth(0).unwrap_or('\n' as u8);
    }
}
