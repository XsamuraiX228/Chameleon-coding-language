use std::collections::HashMap;
use crate::frontend::token::OpType;
#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    Atom(i64),
    Variable(&'a str),
    UnCons(OpType, Box<Expression<'a>>),
    BinCons(OpType, Box<Expression<'a>>, Box<Expression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn evaluate(&self, env: &HashMap<&'a str, i64>) -> Result<i64, String> {
        match self {
            Expression::Atom(n) => Ok(*n),
            Expression::Variable(name) => {
                env.get(*name)
                    .cloned()
                    .ok_or_else(|| format!("Variable '{}' not found", name))
            }
            Expression::UnCons(op, arg) => {
                match op {
                    OpType::Minus => Ok(-arg.evaluate(env)?),
                    OpType::Plus => Ok(arg.evaluate(env)?),
                    OpType::Factorial => Ok(factorial(arg.evaluate(env)?)),
                    OpType::LParen | OpType::RParen => {
                        Err("Parentheses should not appear in evaluation".to_string())
                    }
                    _ => unreachable!(),
                }
            }
            Expression::BinCons(op, lhs, rhs) => {
                match op {
                    OpType::Plus => Ok(lhs.evaluate(env)? + rhs.evaluate(env)?),
                    OpType::Minus => Ok(lhs.evaluate(env)? - rhs.evaluate(env)?),
                    OpType::Multiply => Ok(lhs.evaluate(env)? * rhs.evaluate(env)?),
                    OpType::Divide => {
                        let divisor = rhs.evaluate(env)?;
                        if divisor == 0 {
                            return Err("Division by zero".to_string());
                        } else {
                            Ok(lhs.evaluate(env)? / divisor)
                        }
                    },
                    OpType::Mod => {
                        let divisor = rhs.evaluate(env)?;
                        if divisor == 0 {
                            return Err("Division by zero".to_string());
                        } else {
                            Ok(lhs.evaluate(env)? % divisor)
                        }
                    },
                    OpType::Power => Ok(lhs.evaluate(env)?.pow(rhs.evaluate(env)? as u32)),
                    OpType::LParen | OpType::RParen => {
                        Err("Parentheses should not appear in evaluation".to_string())
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn factorial(mut x: i64) -> i64 {
    let mut sum = 1;
    while x > 1 {
        sum *= x;
        x -= 1;
    }
    sum
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement<'a> {
    Assign { name: &'a str, value: Expression<'a>, }, // Assign the value to variable
    Input {name: &'a str}, // Input the value
    PrintStr(&'a str, bool), // Print strings
    PrintVar(&'a str, bool), // Get value from variables and print it
    If {
        left_value: Expression<'a>, 
        cmp: &'a str, 
        right_value: Expression<'a>, 
        then_block: Vec<Statement<'a>>, 
        else_block: Vec<Statement<'a>>,
    }, // If statement
    While {
        left_value: Expression<'a>, 
        cmp: &'a str, 
        right_value: Expression<'a>, 
        block: Vec<Statement<'a>>
    }, // While loop
    For {
        increment: &'a str, 
        start_idx: Expression<'a>, 
        end_idx: Expression<'a>, 
        block: Vec<Statement<'a>>, 
        step: i64
    }, // For loop
    Label {name: &'a str}, // Mark to control the position where the GOTO will jump
    Goto {label: &'a str}, // Jump to mark in code
    Random {name:&'a str, min: i64, max: i64}, // Set random value to variable
    End, // Stop the program
} 
