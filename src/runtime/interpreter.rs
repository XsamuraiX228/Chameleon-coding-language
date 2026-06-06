use std::collections::HashMap;
use crate::frontend::ast::Statement;

use rand::Rng;
#[derive(Debug, PartialEq)]
pub enum Signal<'a> {
    Continue,
    Jump {label: &'a str},
    Exit,
}

pub struct Environment<'a> {
    map: HashMap<&'a str, i64>
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Environment { map: HashMap::new() }
    }

    pub fn set(&mut self, key: &'a str, value: i64) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &'a str) -> Result<i64, String> {
        self.map
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Runtime Error: variable '{}' not found!", key))
    }
}

pub struct Interpreter<'a> {
    env: Environment<'a>
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self { env: Environment::new() }
    }

    // Scan full code to get the positions of labels, which will be used fo GOTO function
    pub fn pre_scan_labels(&mut self, commands: &[Statement<'a>]) -> HashMap<&'a str, usize>{
        let mut marks: HashMap<&'a str, usize> = HashMap::new();
        for (idx, mark) in commands.iter().enumerate() {
            if let Statement::Label { name } = mark {
                marks.insert(name, idx);
            }
        }
        dbg!(&marks);
        marks
    }

    pub fn execute(&mut self, commands: &[Statement<'a>], labels: &HashMap<&'a str, usize>) -> Result<(), String> {
        let mut command_idx = 0;
        while command_idx < commands.len() {
            match self.execute_single(&commands[command_idx])? {
                Signal::Exit => {
                    break;
                }
                Signal::Jump { label } => {
                    if let Some(&new_idx) = labels.get(label) {
                        command_idx = new_idx;
                        continue;
                    } else {
                        return Err(format!("Runtime Error: Label '{}' not found", label));
                    }
                }
                Signal::Continue => {
                    command_idx += 1
                }
            }
        }
        Ok(())
    }
    // Function returns Option<usize>. 
    // If Some(idx) was returned — GOTO worked and we need to jump to next index
    fn execute_single(
        &mut self, 
        stmt: &Statement<'a>) -> Result<Signal<'a>, String> {
        match stmt {
            Statement::Assign { name, value } => {
                let final_value = (value.evaluate(&self.env.map))?;
                self.env.set(name, final_value);
                Ok(Signal::Continue)
            }

            Statement::Input { name } => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)
                .map_err(|e| format!("Failed to read line: {}", e))?;
                let value: i64 = input.trim().parse()
                .map_err(|e| format!("Invalid integer input: {}",e))?;
                self.env.set(name, value);
                Ok(Signal::Continue)
            }

            Statement::PrintStr(text, newline) => {
                if *newline {println!("{text}")} else {print!("{text}")}
                Ok(Signal::Continue)
            }

            Statement::PrintVar(name, newline) => {
                let val = self.env.get(name)?;
                if *newline {println!("{val}")} else {print!("{val}")}
                Ok(Signal::Continue)
            }

            Statement::Goto { label } => {
                Ok(Signal::Jump { label })
            }
            
            
            Statement::If { 
                left_value, 
                cmp, 
                right_value,
                then_block,
                else_block} => {
                let lhs = left_value.evaluate(&self.env.map)?;
                let rhs = right_value.evaluate(&self.env.map)?;

                let condition = match *cmp {
                    "=" => lhs == rhs,
                    "!" => lhs != rhs,
                    ">" => lhs > rhs,
                    "<" => lhs < rhs,
                    ">=" => lhs >= rhs,
                    "<=" => lhs <= rhs,
                    _ => unreachable!(),
                };

                let block_execute = if condition {then_block} else {else_block};

                for stmt in block_execute {
                    match self.execute_single(stmt)? {
                        Signal::Jump { label } => return Ok(Signal::Jump { label }),
                        Signal::Exit => return Ok(Signal::Exit),
                        Signal::Continue => continue,
                    }
                }
                Ok(Signal::Continue)
            }

            Statement::While { left_value, cmp, right_value, block } => {
                loop {
                    // 1. Get value of experessions from left and right sides
                    let lhs = left_value.evaluate(&self.env.map)?;
                    let rhs = right_value.evaluate(&self.env.map)?;

                    // 2.Check the operator
                    let condition = match *cmp {
                        "="  => lhs == rhs,
                        "!"  => lhs != rhs,
                        "<"  => lhs < rhs,
                        ">"  => lhs > rhs,
                        "<=" => lhs <= rhs,
                        ">=" => lhs >= rhs,
                        _    => unreachable!("Unknown operators in WHILE"),
                    };
                    if !condition {
                        break;
                    }
                    for stmt in block {
                        let signal = self.execute_single(stmt)?;
                        if signal != Signal::Continue {
                            return Ok(signal);
                        }
                    }
                }
                Ok(Signal::Continue)
            }

            Statement::For { increment, start_idx, end_idx, block, step } => {
                let cmp = if *step >= 0 {"<="} else {">="};
                let start = start_idx.evaluate(&self.env.map)?;
                let end = end_idx.evaluate(&self.env.map)?;
                self.env.set(increment, start);
                loop {
                    let i = self.env.get(increment)?;
                    let condition = match cmp {
                        "<=" => i <= end,
                        ">=" => i >= end,
                        _ => unreachable!(),
                    };

                    if !condition {
                        break;
                    }

                    for stmt in block {
                        let signal = self.execute_single(stmt)?;
                        if signal != Signal::Continue {
                            return Ok(signal);
                        }
                    }
                    self.env.set(increment, i + *step);
                }
                Ok(Signal::Continue)
            }

            Statement::Random { name, min, max } => {
                let mut rng = rand::thread_rng();
                let min_val = *min;
                let max_val = *max;

                let random_value: i64 = rng.gen_range(min_val..=max_val);
                
                self.env.set(name, random_value);
                Ok(Signal::Continue)
            }

            Statement::Label { .. } => Ok(Signal::Continue),

            Statement::End => Ok(Signal::Exit),
        }
    }
}