use std::fmt;
use std::io;
use std::io::Write;
use std::process::exit;

#[derive(Debug,PartialEq,Clone,Copy)]
enum TokenType {
    Num,
    Strng,
    BinaryOp,
    Keyword,
    Error,
}

struct Token {
    token_type: TokenType,
    value: f64,
    text: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.token_type {
            TokenType::Num => write!(f, "{}", self.value),
            TokenType::Strng => write!(f, "{}", self.text),
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
        for tok in &self.0 {
            write!(f, "    {}\n", tok);
        }
        Ok(())
    }
}

fn lex(text: &str) -> TokenType {
    match text {
        "+" | "-" | "*" | "/" | "==" => TokenType::BinaryOp,
        "clear" | "reset" | "exit" | "print" => TokenType::Keyword,
        _ => {
            if text.parse::<f64>().is_ok() {
                TokenType::Num
            } else {
                println!("Cannot understand token: {}", text);
                TokenType::Error
            }
        }
    }
}

fn parse_input(text: &str, mut stack: Stack) -> Stack {
    for item in text.split_whitespace() {
        match lex(item) {
            TokenType::Error => break,
            TokenType::Num => {
                let tok = Token{
                    token_type: TokenType::Num,
                    value: item.parse::<f64>().unwrap(),
                    text: "".to_string(),
                };
                stack.push(tok);
            }
            TokenType::Strng => {
                let tok = Token{
                    token_type: TokenType::Strng,
                    value: 0f64,
                    text: item.to_string(),
                };
                stack.push(tok);
            },
            TokenType::BinaryOp => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let Some(result) = match item {
                        "+" => Some(a.value + b.value),
                        "-" => Some(a.value - b.value),
                        "*" => Some(a.value * b.value),
                        "/" => Some(a.value / b.value),
                        _ => {
                            println!("Unknown binary op: {}", item);
                            None
                        }
                    } {
                        let tok = Token{
                            token_type: TokenType::Num,
                            value: result,
                            text: "".to_string(),
                        };
                        stack.push(tok);
                    }
                } else {
                    println!("Insufficient values on stack for binary operation");
                }
            },
            TokenType::Keyword => {
                match item {
                    "clear" | "reset" => stack.clear(),
                    "exit" => exit(0),
                    "print" => {
                        if stack.len() > 0 {
                            let val = stack.pop().unwrap();
                            println!("{}", val);
                        } else {
                            println!("Nothing on the stack to print.");
                        }
                    },
                    _ => println!("Unknown keyword: {}", item),
                }
            }
        }
    }

    return stack;
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut stack: Stack = Stack(vec![]);
    println!("RustPN: A Rust powered RPN calculator.");

    loop {
        stdout.write(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        stack = parse_input(&input, stack);
        println!("Stack (len={}):\n{}", stack.len(), stack);
    }
}
