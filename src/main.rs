use std::fmt;
use std::io;
use std::io::Write;
use std::process::exit;
use std::collections::HashMap;

#[derive(Clone,PartialEq)]
enum TokenType {
    Num,
    Var,
    BinaryOp,
    Assignment,
    Keyword,
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
        self.0.clear();
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
    assignments: HashMap::<String, f64>,
}

impl State {
    fn push_num(&mut self, item: &str) {
        self.stack.push(
            Token{
                token_type: TokenType::Num,
                value: item.parse::<f64>().unwrap(),
                text: "".to_string(),
            }
        );
    }

    fn push_var(&mut self, item: &str) {
        self.stack.push(
            Token{
                token_type: TokenType::Var,
                value: 0f64,
                text: item.to_string(),
            }
        );
    }

    fn do_assignment(&mut self) {
        if self.stack.len() < 2 {
            println!("ERROR: Insufficient values for assignment");
            return;
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        if !(a.token_type == TokenType::Var
            && b.token_type == TokenType::Num) {
            self.stack.push(a);
            self.stack.push(b);
            println!("ERROR: Top vals of stack not suitable for assignment");
            return;
        }
        self.assignments.insert(a.text, b.value);
    }
}

fn lex(text: &str) -> TokenType {
    match text {
        "+" | "-" | "*" | "/" => TokenType::BinaryOp,
        "clear" | "reset" | "exit" | "print" | "swap" | "dup" | "drop" => TokenType::Keyword,
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
    return true;
}

fn parse_input(text: &str, mut state: State) -> State {
    for item in text.split_whitespace() {
        match lex(item) {
            TokenType::Error => break,
            TokenType::Num => state.push_num(&item),
            TokenType::Var => state.push_var(&item),
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
                    },
                } {
                    let tok = Token {
                        token_type: TokenType::Num,
                        value: result,
                        text: "".to_string(),
                    };
                    state.stack.push(tok);
                }
            },
            TokenType::Keyword => {
                match item {
                    "clear" => state.stack.clear(),
                    "reset" => {
                        state.stack.clear();
                        state.assignments.clear();
                    }
                    "exit" => exit(0),
                    "print" => {
                        if !check_sufficient_stack_len(&state.stack, 1) {
                            break;
                        }
                        let val = state.stack.pop().unwrap();
                        println!("{}", val);
                    },
                    "drop" => {
                        if !check_sufficient_stack_len(&state.stack, 1) {
                            break;
                        }
                        state.stack.pop().unwrap();
                    },
                    "dup" => {
                        if !check_sufficient_stack_len(&state.stack, 1) {
                            break;
                        }
                        let val = state.stack.pop().unwrap();
                        state.stack.push(val.clone());
                        state.stack.push(val.clone());
                    },
                    "swap" => {
                        if !check_sufficient_stack_len(&state.stack, 2) {
                            break;
                        }
                        let a = state.stack.pop().unwrap();
                        let b = state.stack.pop().unwrap();
                        state.stack.push(a);
                        state.stack.push(b);
                    },
                    _ => println!("ERROR: Unknown keyword: {}", item),
                }
            }
        }
    }

    return state;
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let mut state = State {
        stack: Stack(vec![]),
        assignments: HashMap::<String, f64>::new(),
    };

    println!("RustPN: A Rust powered RPN calculator.");

    loop {
        stdout.write(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        state = parse_input(&input, state);
        println!("Assigned variables: {:?}", state.assignments);
        println!("Stack (len={}):\n{}", state.stack.len(), state.stack);
    }
}
