#[derive(Debug)]
enum Tokens {
    IntLit(i32),
    Operator(OperatorType),
}

#[derive(Debug)]
enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide
}

fn main() {
    println!("Enter a math equasion: ");
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).unwrap();

    let mut tokens: Vec<Tokens> = Vec::new();

    let chars = buffer.chars().collect::<Vec<char>>();

    for mut i in 0..chars.len() {
        let mut tmp: String = String::new();

        if chars[i].is_ascii_digit() {
            while chars[i].is_ascii_digit() {
                tmp.push(chars[i]);

                i = i + 1;

                continue;
            }

            tokens.push(Tokens::IntLit(tmp.parse().unwrap()));

            tokens.clear();

            i = i - 1;
        }
        else if chars[i] == '+' {
            tokens.push(Tokens::Operator(OperatorType::Plus));
        }
        else if chars[i] == '-' {
            tokens.push(Tokens::Operator(OperatorType::Minus));
        }
        else if chars[i] == '*' {
            tokens.push(Tokens::Operator(OperatorType::Multiply));
        }
        else if chars[i] == '/' {
            tokens.push(Tokens::Operator(OperatorType::Divide));
        }
    }

    println!("{:?}", tokens);
}
