use std::io::Read;

fn main() {
    let mut input = std::fs::File::open("input.txt").unwrap();

    let mut content: String = String::new();

    input.read_to_string(&mut content).unwrap();

    let lines = content.lines();

    let mut tmp: i32 = 0;

    let mut added_values: Vec<i32> = vec![];

    for (i, line) in lines.enumerate() {
        if !line.contains(char::is_numeric) {
            added_values.push(tmp);
            tmp = 0;
            continue;
        } else {
            tmp += line.trim().parse::<i32>().unwrap();
        }
    }

    tmp = 0;

    for i in &added_values {
        if i > &tmp {
            tmp = *i;
        } else {
            continue;
        }
    }

    let no_1 = tmp;

    let mut index = added_values.iter().position(|i| i == &no_1).unwrap();

    added_values.remove(index);

    tmp = 0;

    for i in &added_values {
        if i > &tmp {
            tmp = *i;
        } else {
            continue;
        }
    }

    let no_2 = tmp;

    index = added_values.iter().position(|i| i == &no_2).unwrap();

    added_values.remove(index);

    tmp = 0;

    for i in &added_values {
        if i > &tmp {
            tmp = *i;
        } else {
            continue;
        }
    }

    let no_3 = tmp;

    println!("{}", no_1 + no_2 + no_3);
}
