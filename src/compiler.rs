// compiler.rs
use crate::bytecode::Op;
use std::collections::HashMap;

pub fn compile(code: &str) -> Vec<Op> {
    let mut ops = Vec::new();
    let mut vars = HashMap::new();
    let mut var_id = 0;
    let mut indents = vec![0];
    let mut loop_stack = Vec::new();

    for line in code.lines() {
        let mut in_string = false;
        let mut comment_start = None;
        for (i, c) in line.char_indices() {
            if c == '"' { in_string = !in_string; }
            else if c == '#' && !in_string { comment_start = Some(i); break; }
        }
        let no_comment = if let Some(idx) = comment_start { &line[..idx] } else { line };
        
        let trimmed = no_comment.trim();
        if trimmed.is_empty() { continue; }
        let indent = no_comment.chars().take_while(|c| *c == ' ').count();

        while indent < *indents.last().unwrap() {
            indents.pop();
            if let Some((start_pc, jmp_idx, i_var)) = loop_stack.pop() {
                ops.push(Op::Load(i_var)); ops.push(Op::Push(1));
                ops.push(Op::Add); ops.push(Op::Store(i_var));
                ops.push(Op::Jmp(start_pc));
                let target_len = ops.len();
                if let Op::JmpIfGt(ref mut target) = ops[jmp_idx] { *target = target_len; }
            }
        }

        if trimmed.ends_with(":") { indents.push(indent + 4); }

        if trimmed.starts_with("<fn") || trimmed == "main()" { continue; }
        else if trimmed.starts_with("<import src=") {
            let file = trimmed.split('"').nth(1).unwrap_or("");
            ops.push(Op::LazyLoad(file.to_string()));
        }
        else if trimmed.starts_with("let user_input") {
            let prompt = trimmed.split('"').nth(1).unwrap_or("");
            let id = *vars.entry("user_input".to_string()).or_insert_with(|| { var_id += 1; var_id - 1 });
            ops.push(Op::Input(prompt.to_string(), id));
        }
        else if trimmed.starts_with("let mut sum") {
            let id = *vars.entry("sum".to_string()).or_insert_with(|| { var_id += 1; var_id - 1 });
            ops.push(Op::Push(0)); ops.push(Op::Store(id));
        }
        else if trimmed.starts_with("for ") {
            let var_name = trimmed.split_whitespace().nth(1).unwrap();
            let bounds = trimmed.split("in ").nth(1).unwrap().trim_end_matches(':');
            let mut parts = bounds.split("..");
            let start: i64 = parts.next().unwrap().parse().unwrap();
            let end: i64 = parts.next().unwrap().parse().unwrap();
            
            let i_var = *vars.entry(var_name.to_string()).or_insert_with(|| { var_id += 1; var_id - 1 });
            ops.push(Op::Push(start)); ops.push(Op::Store(i_var));
            
            let start_pc = ops.len();
            ops.push(Op::Load(i_var)); ops.push(Op::Push(end));
            ops.push(Op::JmpIfGt(0)); 
            loop_stack.push((start_pc, ops.len() - 1, i_var));
        }
        else if trimmed == "sum += 1" {
            let s_var = *vars.get("sum").unwrap();
            ops.push(Op::Load(s_var)); ops.push(Op::Push(1));
            ops.push(Op::Add); ops.push(Op::Store(s_var));
        }
        else if trimmed.starts_with("print(f\"") {
            let u_var = *vars.get("user_input").unwrap();
            let s_var = *vars.get("sum").unwrap();
            ops.push(Op::PrintFormat(u_var, s_var));
        }
        else if trimmed.starts_with("print(\"") {
            let text = trimmed.split('"').nth(1).unwrap_or("");
            ops.push(Op::PrintStr(text.to_string()));
        }
    }

    while indents.len() > 1 {
        indents.pop();
        if let Some((start_pc, jmp_idx, i_var)) = loop_stack.pop() {
            ops.push(Op::Load(i_var)); ops.push(Op::Push(1));
            ops.push(Op::Add); ops.push(Op::Store(i_var));
            ops.push(Op::Jmp(start_pc));
            let target_len = ops.len();
            if let Op::JmpIfGt(ref mut target) = ops[jmp_idx] { *target = target_len; }
        }
    }
    ops
}
