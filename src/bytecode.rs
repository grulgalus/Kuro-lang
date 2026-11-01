// bytecode.rs
#[derive(Debug, Clone)]
pub enum Op {
    Push(i64), Load(usize), Store(usize), Add,
    PrintStr(String), Input(String, usize),
    PrintFormat(usize, usize), JmpIfGt(usize), Jmp(usize), LazyLoad(String),
}
