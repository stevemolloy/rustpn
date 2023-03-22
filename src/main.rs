use std::io;
use std::io::Write;
use std::process::exit;

#[derive(Debug,PartialEq,Clone,Copy)]
enum TokenType {
    Num,
    BinaryOp,
    Keyword,
    Error,
}

type Stack = Vec<f64>;

fn lex(text: &str) -> TokenType {
    match text {
        "+" | "-" | "*" | "/" | "==" => TokenType::BinaryOp,
        "clear" | "reset" | "exit" => TokenType::Keyword,
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
            TokenType::Num => {stack.push(item.parse::<f64>().unwrap())},
            TokenType::BinaryOp => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let Some(result) = match item {
                        "+" => Some(a + b),
                        "-" => Some(a - b),
                        "*" => Some(a * b),
                        "/" => Some(a / b),
                        _ => {
                            println!("Unknown binary op: {}", item);
                            None
                        }
                    } {
                        stack.push(result);
                    }
                } else {
                    println!("Insufficient values on stack for binary operation");
                }
            },
            TokenType::Keyword => {
                match item {
                    "clear" | "reset" => stack.clear(),
                    "exit" => exit(0),
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
    let mut stack: Stack = vec![];
    println!("RustPN: A Rust powered RPN calculator.");

    loop {
        stdout.write(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        stack = parse_input(&input, stack);
        println!("Stack (len={}): {:#?}", stack.len(), stack);
    }
}
