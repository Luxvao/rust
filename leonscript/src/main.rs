use std::{process::exit, path::PathBuf, io::Read};

#[derive(Debug, Clone)]
enum Tokens {
    Let,
    Fu,
    For,
    In,
    If,
    Else,
    Semi,
    OpenParenth,
    CloseParenth,
    OpenSquigly,
    CloseSquigly,
    IntLit(i32),
    Name(String),
    StrLit(String),
    CharLit(char),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            println!("Please specify a path to a *.ls file!");
            exit(1);
        }
    };

    println!("{}", file_path);
    
    let mut input_file = std::fs::File::open(PathBuf::from(file_path)).expect("Couldn't find the input file!");

    let mut contents = String::new();

    input_file.read_to_string(&mut contents).unwrap();

    let mut token = String::new();

    let mut tokens: Vec<Tokens> = Vec::new();

    for c in contents.chars() {
        if c.is_whitespace() {
            token.clear();
            continue;
        }

        match token.trim() {
            "let" => {
                tokens.push(Tokens::Let);
                token.clear();
                continue;
            },
            "fu" => {
                tokens.push(Tokens::Fu);
                token.clear();
                continue;
            },
            "for" => {
                tokens.push(Tokens::For);
                token.clear();
                continue;
            },
            "in" => {
                tokens.push(Tokens::In);
                token.clear();
                continue;
            },
            "if" => {
                tokens.push(Tokens::If);
                token.clear();
                continue;
            },
            "else" => {
                tokens.push(Tokens::Else);
                token.clear();
                continue;
            },
            anything => {
                match anything.parse() {
                    Ok(val) => {
                        tokens.push(Tokens::IntLit(val));
                        token.clear();
                        continue;
                    },
                    Err(_) => {
                        tokens.push(Tokens::Name(anything.to_owned()));
                        token.clear();
                        continue;
                    },
                }
            }
        }
    }

    println!("{:?}", tokens);
}
