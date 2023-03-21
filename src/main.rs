#[derive(Debug,PartialEq,Clone,Copy)]
enum TokenType {
    Num,
    BinaryOp,
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

fn parse_input(text: String) -> Vec<f64> {
    let mut stack: Vec<f64> = vec![];

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
            }
        }
    }

    return stack;
}

fn main() {
    let input = "1 200 + 10 /".to_string();
    // println!("Input string = {}", input);

    let stack = parse_input(input);
    assert!(!stack.is_empty(), "Cannot work with an empty stack");
    assert!(stack.len() == 1, "Unprocessed elements in the stack");

    println!("Stack at end = {:?}", stack);
}
