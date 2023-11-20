use std::{process::exit, io::{Read, Write}};

fn main() {
    println!("Input the number of videos:");
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let num_of_videos: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {}", err);
            exit(1);
        }
    };

    let mut output_file = match std::fs::File::create("output/video.ts") {
        Ok(file) => file,
        Err(err) => {
            println!("Error: {}", err);
            exit(1);
        }
    };

    for i in 0..num_of_videos {
        let mut input_file = match std::fs::File::open(format!("input/{}.ts", i).as_str()) {
            Ok(file) => file,
            Err(err) => {
                println!("Error: {}", err);
                exit(1);
            }
        };

        let mut buffer: Vec<u8> = Vec::new();

        let _ = input_file.read_to_end(&mut buffer).unwrap();

        let _ = output_file.write_all(&buffer).unwrap();
    }
}
