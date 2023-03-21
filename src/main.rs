#[derive(Debug,Clone)]
enum TokenType {
    Num,
    BinaryOp,
}

#[derive(Debug,Clone)]
struct Token {
    token_type: TokenType,
    text: String,
}

fn lex(text: &str) -> TokenType {
    match text {
        "+" => TokenType::BinaryOp,
        "-" => TokenType::BinaryOp,
        "*" => TokenType::BinaryOp,
        "/" => TokenType::BinaryOp,
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

    for i in text.split_whitespace() {
        let new_token = Token{
            token_type: lex(i), 
            text: i.to_string()
        };
        stack.push(new_token);
    }

    return stack;
}

fn resolve_stack(stack: &mut Vec<Token>) -> f64 {
    while stack.len() > 1 {
        let token = stack.pop().unwrap();
        match token.token_type {
            TokenType::Num => {
                return token.text.parse::<f64>().unwrap();
            }
            TokenType::BinaryOp => {
                let b = resolve_stack(stack);
                let a = resolve_stack(stack);
                let result = match token.text.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => panic!("Unknown binary op: {}", token.text),
                };
                return result;
            },
        }
    }
    return stack.pop().unwrap().text.parse::<f64>().unwrap();
}

fn main() {
    let input = "2 3 / 1 + 3 *".to_string();
    // println!("Input string = {}", input);

    let mut stack = parse_input(input);
    assert!(!stack.is_empty(), "Cannot work with an empty stack");

    let answer = resolve_stack(&mut stack);
    println!("{:?}", answer);
}
