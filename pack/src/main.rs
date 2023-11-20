use std::{process::exit, path::PathBuf, io::{Read, Write, Seek, SeekFrom}};

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let mut output_name: Option<String> = None;
    let verbose: bool;

    match args.contains(&String::from("-o")) {
        true => {
            let index: usize = args.iter().position(|i| i.trim() == "-o").unwrap();

            output_name = Some(args.get(index + 1).unwrap().clone());
            args.remove(index);
            args.remove(index);
        }
        false => (),
    }

    match args.contains(&String::from("-h")) {
        true => {
            help();
            exit(0);
        }
        false => ()
    }

    match args.contains(&String::from("-v")) {
        true => {
            verbose = true;
            args.remove(args.iter().position(|i| i.trim() == "-v").unwrap());
        }
        false => verbose = false,
    }

    match args.contains(&String::from("-x")) {
        true => {
            args.remove(args.iter().position(|i| i.trim() == "-x").unwrap());
            unpack(&args, output_name, verbose);
        },
        false => pack(args, output_name, verbose),
    }
}

fn help() {
    println!("[*] Pack - 1.0\n
        [+] Usage:\n
            pack <file(s)> <flag(s)>

        [+] Flags:\n
            -h - Prints this message\n
            -o - Specifies the output file/directory\n
            -x - Extracts the contents of a pack file\n");
}

fn unpack(args: &Vec<String>, output_name: Option<String>, verbose: bool) {
    let mut bytes: [u8; 4] = [0; 4];

    let mut verbose_name = String::new();

    if args.len() < 2 {
        println!("A file should be supplied via command line arguments!");
        
        exit(1);
    }

    let mut file = std::fs::File::open(PathBuf::from(args[1].clone())).expect("File not found!");

    file.read_exact(&mut bytes).unwrap();
    
    if bytes != [b'p', b'a', b'c', b'k'] {
        println!("Magic number not found - archive unsupported!");
        exit(1);
    }

    match &output_name {
        Some(output_name) => {
            std::fs::create_dir(PathBuf::from(output_name)).unwrap();
            verbose_name = output_name.clone();

            verbose_name.push('/');
        }
        None => (),
    }
    
    if verbose {
        print!("{} => ", args[1].clone());
    }

    loop {
        let mut name_size: [u8; 1] = [0; 1];

        file.read_exact(&mut name_size).unwrap();

        let mut file_name = vec![0; name_size[0].into()];
        
        file.read_exact(&mut file_name).unwrap();
        
        if verbose {
            print!("{}{} ", verbose_name, String::from_utf8(file_name.clone()).unwrap());
        }

        let mut file_size: [u8; 8] = [0; 8];

        file.read_exact(&mut file_size).unwrap();

        let mut output_file_contents = vec![0; usize::from_le_bytes(file_size)];
    
        file.read_exact(&mut output_file_contents).unwrap();

        let mut output_file = match &output_name {
            Some(output_name) => std::fs::File::create(PathBuf::from(format!("{}/{}", output_name, String::from_utf8(file_name).unwrap()))).unwrap(),
            None => std::fs::File::create(PathBuf::from(String::from_utf8(file_name).unwrap())).unwrap(),
        };

        output_file.write_all(&output_file_contents).unwrap();

        let mut ending_bytes: [u8; 4] = [0; 4];

        file.read_exact(&mut ending_bytes).unwrap();

        if ending_bytes == [b'k', b'c', b'a', b'p'] {
            break;
        }
    }
    
    if verbose {
        println!("");
    }
}

fn pack(args: Vec<String>, output_name: Option<String>, verbose: bool) {
    let mut verbose_name = String::new();

    if args.len() < 2 {
        println!("(A) file(s) should be supplied via command line arguments!");

        exit(1);
    }

    let mut args = args.clone();

    args.remove(0);

    let mut output_file = match output_name {
        Some(output_name) => {
            verbose_name = output_name.clone();
            std::fs::File::create(PathBuf::from(output_name)).unwrap()
        }
        None => std::fs::File::create(PathBuf::from("out.pck")).unwrap()
    };

    output_file.write(&[b'p', b'a', b'c', b'k']).unwrap();
    
    for file_name in args.iter_mut() {
        let chars = file_name.clone().chars().collect::<Vec<char>>();
        
        if chars[0] == '.' {
            if chars[1] == '/' || chars[1] == '\\' {
                file_name.remove(0);
                file_name.remove(0);
            }
        }

        println!("Filename: {}", file_name);

        let mut input_file = std::fs::File::open(PathBuf::from(file_name.clone())).expect("File not found!");

        let mut input_file_contents: Vec<u8> = Vec::new();
        
        if verbose {
            print!("{} ", file_name);
        }

        input_file.read_to_end(&mut input_file_contents).unwrap();
        
        output_file.write_all(&[file_name.len() as u8]).unwrap();

        output_file.write_all(file_name.trim().as_bytes()).unwrap();

        output_file.write_all(&input_file_contents.len().to_le_bytes()).unwrap();

        output_file.write_all(&input_file_contents).unwrap();

        output_file.write_all(&[0; 4]).unwrap();
    }

    output_file.seek(SeekFrom::Current(-4)).unwrap();

    output_file.write_all(&[b'k', b'c', b'a', b'p']).unwrap();

    if verbose {
        print!("=> {}", verbose_name);
    }
}
