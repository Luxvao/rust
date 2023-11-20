fn main() {
    println!("Creating directories...");

    for i in 1..13 {
        let _ = std::fs::create_dir(format!("C:\\Users\\Bor\\Projects\\VideoDownloader\\video{i}"));
    }
}
