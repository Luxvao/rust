fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("{}", args.get(0).unwrap());
}