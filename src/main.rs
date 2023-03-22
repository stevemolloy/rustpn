use std::io;
use std::io::Write;
use std::process::exit;

#[derive(Debug,PartialEq,Clone,Copy)]
enum TokenType {
    Num,
    BinaryOp,
    Keyword,
}

fn lex(text: &str) -> TokenType {
    match text {
        "+" | "-" | "*" | "/" | "==" => TokenType::BinaryOp,
        "clear" | "reset" | "exit" => TokenType::Keyword,
        _ => {
            if text.parse::<f64>().is_ok() {
                TokenType::Num
            } else {
                panic!("Unrecognised token: {}", text);
            }
        }
    }
}

fn parse_input(text: &str, mut stack: Vec<f64>) -> Vec<f64> {
    for item in text.split_whitespace() {
        match lex(item) {
            TokenType::Num => {stack.push(item.parse::<f64>().unwrap())},
            TokenType::BinaryOp => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match item {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => panic!("Unknown binary op: {}", item),
                };
                stack.push(result);
            },
            TokenType::Keyword => {
                match item {
                    "clear" | "reset" => stack.clear(),
                    "exit" => exit(0),
                    _ => panic!("Unknown keyword: {}", item),
                }
            }
        }
    }

    return stack;
}

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut stack: Vec<f64> = vec![];
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
