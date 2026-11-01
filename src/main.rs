// main.rs
mod bytecode; mod compiler; mod vm;
use bytecode::Op; use std::{env, fs, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Pouziti:\n  ./koru run <skript.kr>   - Spusti kod v interpretu\n  ./koru build <skript.kr> - Zkompiluje do nativni binarky");
        return;
    }
    let mode = &args[1]; let filename = &args[2];
    let content = fs::read_to_string(filename).expect("Nelze cist soubor");
    let bytecode = compiler::compile(&content);

    if mode == "run" {
        println!("--- KORU JIT RUN ---");
        vm::VM::new(bytecode).run();
    } else if mode == "build" {
        build_executable(&bytecode, filename);
    } else { println!("Neznamy prikaz: {}", mode); }
}

fn build_executable(bytecode: &[Op], source_file: &str) {
    let mut rust_code = String::from(r#"
use std::collections::HashMap; use std::io::{self, Write};
#[derive(Clone)] pub enum Op { Push(i64), Load(usize), Store(usize), Add, PrintStr(String), Input(String, usize), PrintFormat(usize, usize), JmpIfGt(usize), Jmp(usize), LazyLoad(String) }
pub struct VM { code: Vec<Op>, stack: Vec<i64>, mem_int: HashMap<usize, i64>, mem_str: HashMap<usize, String> }
impl VM {
    pub fn new(code: Vec<Op>) -> Self { Self { code, stack: Vec::new(), mem_int: HashMap::new(), mem_str: HashMap::new() } }
    pub fn run(&mut self) {
        let mut pc = 0;
        while pc < self.code.len() {
            match &self.code[pc] {
                Op::LazyLoad(file) => println!(">>> [LAZY LOAD] {}", file),
                Op::Push(val) => self.stack.push(*val),
                Op::Load(idx) => self.stack.push(*self.mem_int.get(idx).unwrap_or(&0)),
                Op::Store(idx) => { let val = self.stack.pop().unwrap(); self.mem_int.insert(*idx, val); },
                Op::Add => { let b = self.stack.pop().unwrap(); let a = self.stack.pop().unwrap(); self.stack.push(a + b); },
                Op::PrintStr(s) => println!("{}", s),
                Op::Input(prompt, idx) => {
                    print!("{}", prompt); io::stdout().flush().unwrap();
                    let mut s = String::new(); io::stdin().read_line(&mut s).unwrap(); 
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
fn main() { let bytecode = vec![
"#);

    for op in bytecode {
        let op_str = match op {
            Op::Push(v) => format!("Op::Push({})", v), Op::Load(i) => format!("Op::Load({})", i),
            Op::Store(i) => format!("Op::Store({})", i), Op::Add => "Op::Add".to_string(),
            Op::PrintStr(s) => format!("Op::PrintStr({:?}.to_string())", s),
            Op::Input(p, i) => format!("Op::Input({:?}.to_string(), {})", p, i),
            Op::PrintFormat(u, s) => format!("Op::PrintFormat({}, {})", u, s),
            Op::JmpIfGt(t) => format!("Op::JmpIfGt({})", t), Op::Jmp(t) => format!("Op::Jmp({})", t),
            Op::LazyLoad(f) => format!("Op::LazyLoad({:?}.to_string())", f),
        };
        rust_code.push_str(&format!("    {},\n", op_str));
    }
    rust_code.push_str("]; VM::new(bytecode).run(); }\n");

    let out_rs = source_file.replace(".kr", ".rs");
    let out_bin = source_file.replace(".kr", "");
    std::fs::write(&out_rs, rust_code).expect("Chyba zapisu");
    println!(">> Zkompilovano do mezikodu. Spoustim AOT Kompilaci (rustc -O)...");
    
    if Command::new("rustc").arg("-O").arg(&out_rs).arg("-o").arg(&out_bin).status().expect("Selhal rustc").success() {
        std::fs::remove_file(&out_rs).unwrap_or(());
        println!(">> [BUILD OK] Nativni binarka uspesne vytvorena: ./{}", out_bin);
    }
}
