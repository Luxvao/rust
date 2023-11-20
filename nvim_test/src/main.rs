fn main() {
    let mut input_buffer = String::new();

    std::io::stdin().read_line(&mut input_buffer).unwrap();

    if input_buffer.trim() == "hi from neovide" {
        println!("W I set up the LSP :)))");
    }
}
