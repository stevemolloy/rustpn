use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;

use crossterm::{cursor, terminal, ExecutableCommand};

#[derive(Clone, PartialEq)]
enum TokenType {
    Num,
    Var,
    BinaryOp,
    Assignment,
    Keyword,
    StackFold,
    Error,
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    value: f64,
    text: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.token_type {
            TokenType::Num => write!(f, "{}", self.value),
            TokenType::Var => write!(f, "{}", self.text),
            _ => unreachable!("Trying to disp a non-string/value"),
        }
    }
}

struct Stack(pub Vec<Token>);

impl Stack {
    fn push(&mut self, item: Token) {
        self.0.push(item);
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn pop(&mut self) -> Option<Token> {
        self.0.pop()
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, tok| {
            result.and_then(|_| writeln!(f, "{}", tok))
        })
    }
}

struct State {
    stack: Stack,
    assignments: HashMap<String, f64>,
    running: bool,
}

impl State {
    fn push_num(&mut self, item: &str) {
        self.stack.push(Token {
            token_type: TokenType::Num,
            value: item.parse::<f64>().unwrap(),
            text: "".to_string(),
        });
    }

    fn push_var(&mut self, item: &str) {
        self.stack.push(Token {
            token_type: TokenType::Var,
            value: 0f64,
            text: item.to_string(),
        });
    }

    fn do_assignment(&mut self) {
        if self.stack.len() < 2 {
            println!("ERROR: Insufficient values for assignment");
            return;
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        if !(self.assignments.contains_key(&b.text)
            || a.token_type == TokenType::Var && b.token_type == TokenType::Num)
        {
            self.stack.push(a);
            self.stack.push(b);
            println!("ERROR: Top vals of stack not suitable for assignment");
            return;
        }
        if b.token_type == TokenType::Num {
            self.assignments.insert(a.text, b.value);
        } else if b.token_type == TokenType::Var {
            self.assignments.insert(a.text, self.assignments[&b.text]);
        }
    }

    fn check_stacksvars_assigned(&self) -> bool {
        for ele in self.stack.0.iter() {
            if ele.token_type == TokenType::Var && !self.assignments.contains_key(&ele.text) {
                return false;
            }
        }
        true
    }
}

fn lex(text: &str) -> TokenType {
    match text {
        "+" | "-" | "*" | "/" => TokenType::BinaryOp,
        "clear" | "reset" | "exit" | "print" => TokenType::Keyword,
        "swap" | "dup" | "drop" => TokenType::Keyword,
        "prod"| "sum" => TokenType::StackFold,
        "=" => TokenType::Assignment,
        _ => {
            if text.parse::<f64>().is_ok() {
                TokenType::Num
            } else if text.chars().all(char::is_alphanumeric) {
                TokenType::Var
            } else {
                println!("ERROR: Cannot understand token: {}", text);
                TokenType::Error
            }
        }
    }
}

fn check_sufficient_stack_len(stack: &Stack, length: usize) -> bool {
    if stack.len() < length {
        println!("ERROR: Insufficient values on stack.");
        return false;
    }
    true
}

fn parse_input(text: &str, mut state: State) -> State {
    for item in text.split_whitespace() {
        match lex(item) {
            TokenType::Error => break,
            TokenType::Num => state.push_num(item),
            TokenType::Var => state.push_var(item),
            TokenType::Assignment => state.do_assignment(),
            TokenType::BinaryOp => {
                if !check_sufficient_stack_len(&state.stack, 2) {
                    break;
                }
                let b = state.stack.pop().unwrap();
                if b.token_type == TokenType::Var && !state.assignments.contains_key(&b.text) {
                    println!("ERROR: Var {} has not yet been assigned a value", b.text);
                    state.stack.push(b);
                    break;
                }
                let a = state.stack.pop().unwrap();
                if a.token_type == TokenType::Var && !state.assignments.contains_key(&a.text) {
                    println!("ERROR: Var {} has not yet been assigned a value", a.text);
                    state.stack.push(a);
                    state.stack.push(b);
                    break;
                }
                let val1 = match a.token_type {
                    TokenType::Num => a.value,
                    TokenType::Var => state.assignments[&a.text],
                    _ => unreachable!(),
                };
                let val2 = match b.token_type {
                    TokenType::Num => b.value,
                    TokenType::Var => state.assignments[&b.text],
                    _ => unreachable!(),
                };
                if let Some(result) = match item {
                    "+" => Some(val1 + val2),
                    "-" => Some(val1 - val2),
                    "*" => Some(val1 * val2),
                    "/" => Some(val1 / val2),
                    _ => {
                        println!("ERROR: Unknown binary op: {}", item);
                        None
                    }
                } {
                    let tok = Token {
                        token_type: TokenType::Num,
                        value: result,
                        text: "".to_string(),
                    };
                    state.stack.push(tok);
                }
            }
            TokenType::Keyword => match item {
                "clear" => state.stack.clear(),
                "reset" => {
                    state.stack.clear();
                    state.assignments.clear();
                }
                "exit" => {
                    state.running = false;
                }
                "print" => {
                    if !check_sufficient_stack_len(&state.stack, 1) {
                        break;
                    }
                    let val = state.stack.pop().unwrap();
                    println!("{}", val);
                }
                "drop" => {
                    if !check_sufficient_stack_len(&state.stack, 1) {
                        break;
                    }
                    state.stack.pop().unwrap();
                }
                "dup" => {
                    if !check_sufficient_stack_len(&state.stack, 1) {
                        break;
                    }
                    let val = state.stack.pop().unwrap();
                    state.stack.push(val.clone());
                    state.stack.push(val.clone());
                }
                "swap" => {
                    if !check_sufficient_stack_len(&state.stack, 2) {
                        break;
                    }
                    let a = state.stack.pop().unwrap();
                    let b = state.stack.pop().unwrap();
                    state.stack.push(a);
                    state.stack.push(b);
                }
                _ => println!("ERROR: Unknown keyword: {}", item),
            }
            TokenType::StackFold => match item {
                "sum" => {
                    if !state.check_stacksvars_assigned() {
                        println!("ERROR: Attempting to use stack containing unassigned variables.");
                        break;
                    }
                    let result = state.stack.0.iter().fold(0f64, |acc, elem| -> f64 {
                        let newvalue = match elem.token_type {
                            TokenType::Num => elem.value,
                            TokenType::Var => state.assignments[&elem.text],
                            _ => unreachable!("Not a num or a var"),
                        };
                        acc + newvalue
                    });
                    state.stack.clear();
                    let tok = Token {
                        token_type: TokenType::Num,
                        value: result,
                        text: "".to_string(),
                    };
                    state.stack.push(tok);
                }
                "prod" => {
                    if !state.check_stacksvars_assigned() {
                        println!("ERROR: Attempting to use stack containing unassigned variables.");
                        break;
                    }
                    let result = state.stack.0.iter().fold(1f64, |acc, elem| -> f64 {
                        let newvalue = match elem.token_type {
                            TokenType::Num => elem.value,
                            TokenType::Var => state.assignments[&elem.text],
                            _ => unreachable!("Not a num or a var"),
                        };
                        acc * newvalue
                    });
                    state.stack.clear();
                    let tok = Token {
                        token_type: TokenType::Num,
                        value: result,
                        text: "".to_string(),
                    };
                    state.stack.push(tok);
                }
                _ => println!("ERROR: Unknown keyword: {}", item),
            },
        }
    }

    state
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap()
        .execute(cursor::MoveTo(0, 0))
        .unwrap();

    let mut state = State {
        stack: Stack(vec![]),
        assignments: HashMap::<String, f64>::new(),
        running: true,
    };

    println!("RustPN: A Rust powered RPN calculator.");

    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();

        stdout.execute(cursor::SavePosition).unwrap();

        stdout.execute(cursor::MoveTo(40, 0)).unwrap();
        println!("|   Stack");
        stdout.execute(cursor::MoveTo(40, 1)).unwrap();
        println!("|===========");
        for loc in 0..15 {
            stdout.execute(cursor::MoveTo(40, loc + 2)).unwrap();
            stdout
                .execute(terminal::Clear(terminal::ClearType::UntilNewLine))
                .unwrap();
            if usize::from(loc) < state.stack.len() {
                state.stack.0.reverse();
                print!("| {}", state.stack.0[loc as usize]);
                state.stack.0.reverse();
            } else {
                print!("|");
            }
        }

        stdout.execute(cursor::MoveTo(52, 0)).unwrap();
        println!("|     Variables     |");
        stdout.execute(cursor::MoveTo(52, 1)).unwrap();
        println!("|===================|");
        for loc in 0..15 {
            stdout.execute(cursor::MoveTo(52, loc + 2)).unwrap();
            stdout
                .execute(terminal::Clear(terminal::ClearType::UntilNewLine))
                .unwrap();
            print!("|                   |");
        }
        let mut loc = 2;
        for (key, value) in state.assignments.iter() {
            stdout.execute(cursor::MoveTo(54, loc)).unwrap();
            print!("{key} = {value:.*}", 10, value = value);
            loc += 1;
        }

        stdout.execute(cursor::RestorePosition).unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        state = parse_input(&input, state);

        if !state.running {
            break;
        }
    }

    stdout
        .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
        .unwrap();
}
