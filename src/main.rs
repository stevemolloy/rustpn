#[derive(Debug,PartialEq,Clone,Copy)]
enum TokenType {
    Num,
    BinaryOp,
}

#[derive(Debug)]
struct Token {
    text: String,
}

fn lex(text: &str) -> TokenType {
    match text {
        "+" | "-" | "*" | "/" | "==" => TokenType::BinaryOp,
        _ => {
            if text.parse::<f64>().is_ok() {
                TokenType::Num
            } else {
                panic!("Unrecognised token: {}", text);
            }
        }
    }
}

fn parse_input(text: String) -> Vec<Token> {
    let mut stack: Vec<Token> = vec![];

    for item in text.split_whitespace() {
        match lex(item) {
            TokenType::Num => {
                stack.push(Token {
                    text: item.to_string()
                })
            },
            TokenType::BinaryOp => {
                let b = stack.pop().unwrap().text.parse::<f64>().unwrap();
                let a = stack.pop().unwrap().text.parse::<f64>().unwrap();
                let result = match item {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => panic!("Unknown binary op: {}", item),
                };
                stack.push(Token {
                    text: result.to_string()
                })
            }
        }
    }

    return stack;
}

fn main() {
    let input = "110 1 200 + 10 /".to_string();
    // println!("Input string = {}", input);

    let stack = parse_input(input);
    assert!(!stack.is_empty(), "Cannot work with an empty stack");
    assert!(stack.len() == 1, "Unprocessed elements in the stack");

    println!("Stack at end = {:?}", stack);
}
