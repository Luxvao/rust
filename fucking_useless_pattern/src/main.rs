fn main() {
    let mut num = 25;

    for i in 0..5 {
        for j in 6..i {
            print!("{num} ");
            num -= 1;
        }
        print!("\n");
    }
}
