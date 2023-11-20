use std::time::Duration;

use rand::Rng;
use reqwest::blocking::Client;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct RequestBody<'a> {
    content: &'a str,
}

fn main() {
    let mut token: String = String::new();
    let mut message: String = String::new();
    let mut channel_id: String = String::new();

    println!("Enter token: ");

    let _ = std::io::stdin().read_line(&mut token);

    println!("Enter channel ID: ");

    let _ = std::io::stdin().read_line(&mut channel_id);

    println!("Enter message to spam: ");

    let _ = std::io::stdin().read_line(&mut message);
    
    let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id.trim());

    let body: RequestBody = RequestBody {
        content: message.trim(),
    };

    let serialized = serde_json::to_string(&body).unwrap();

    let client = Client::new();
    
    loop {
       let response = client
            .post(url.trim())
            .body(serialized.clone())
            .header("Authorization", token.trim())
            .header("User-Agent", "RokonjaAnnoyer10000")
            .header("Content-Type", "application/json")
            .header("Host", "discord.com")
            .send()
            .unwrap();
    
        let rand_int: i32 = rand::thread_rng().gen_range((0..4));

        std::thread::sleep(Duration::from_secs(rand_int as u64))
    }
}
