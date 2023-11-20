fn main() {
    println!("Enter the string to be obfuscated: ");
    let mut input_string = String::new();

    std::io::stdin().read_line(&mut input_string).unwrap();

    println!("Enter the offset: ");
    let mut input_offset_tmp = String::new();

    std::io::stdin().read_line(&mut input_offset_tmp).unwrap();

    let mut input_offset: i32 = input_offset_tmp.trim().parse().unwrap();

    if input_offset < 32 {
        input_offset = 32;
    }
    if input_offset > 126 {
        input_offset = 126;
    }

    let output: Vec<u8> = input_string.chars().map(|i| i as u8 + input_offset as u8).collect();

    println!("Output: {}", String::from_utf8_lossy(&output));
}
