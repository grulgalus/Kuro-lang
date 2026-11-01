// vm.rs
use crate::bytecode::Op;
use std::collections::HashMap;
use std::io::{self, Write};

pub struct VM {
    code: Vec<Op>, stack: Vec<i64>,
    mem_int: HashMap<usize, i64>, mem_str: HashMap<usize, String>,
}

impl VM {
    pub fn new(code: Vec<Op>) -> Self {
        Self { code, stack: Vec::new(), mem_int: HashMap::new(), mem_str: HashMap::new() }
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        while pc < self.code.len() {
            match &self.code[pc] {
                Op::LazyLoad(file) => println!(">>> [LAZY LOAD] Modul '{}' pozadovan.", file),
                Op::Push(val) => self.stack.push(*val),
                Op::Load(idx) => self.stack.push(*self.mem_int.get(idx).unwrap_or(&0)),
                Op::Store(idx) => { let val = self.stack.pop().unwrap(); self.mem_int.insert(*idx, val); },
                Op::Add => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a + b); },
                Op::PrintStr(s) => println!("{}", s),
                Op::Input(prompt, idx) => {
                    print!("{}", prompt); io::stdout().flush().unwrap();
                    let mut s = String::new(); 
                    io::stdin().read_line(&mut s).unwrap(); 
                    self.mem_str.insert(*idx, s.trim().to_string());
                },
                Op::PrintFormat(s_idx, i_idx) => {
                    let s_val = self.mem_str.get(s_idx).unwrap_or(&String::new()).clone();
                    let i_val = self.mem_int.get(i_idx).unwrap_or(&0);
                    println!("Ahoj {}, tajne cislo je {}", s_val, i_val);
                },
                Op::JmpIfGt(target) => { let limit = self.stack.pop().unwrap(); if self.stack.pop().unwrap() > limit { pc = *target; continue; } },
                Op::Jmp(target) => { pc = *target; continue; },
            }
            pc += 1;
        }
    }
}
