use reqwest::blocking::*;

fn main() {
    let client = Client::new();

    loop {
        client.post("127.0.0.1").send().unwrap();
    }
}
