use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("input2.txt").unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let lines = content.lines();

    let mut combined_values = 0;

    let mut past_chars: Vec<char> = Vec::new();

    for line in lines {
        let half1 = &line[..line.len()/2];
        let half2 = &line[line.len()/2..];
        
        for char in half1.chars() {
            let tmp = past_chars.iter().filter(|i| **i == char).collect::<Vec<&char>>();

            if tmp.len() > 0 {
                continue;
            }

            for inner_char in half2.chars() {
                if char == inner_char {
                    past_chars.push(char);
                    println!("{}", char);
                }
                else {
                    continue;
                }
            }
        }
    }
}
