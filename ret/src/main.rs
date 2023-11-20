use std::{process::exit, path::PathBuf};
use reqwest::

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("ERROR: Incorrect usage!");
        exit(1);
    }

    match args[1].trim() {
        "-h" => help(),
        "--help" => help(),
        "update" => update(),
        "check" => check(),
        "set-home" => sethome(&args),
        "install" => install(),
        _ => {
            println!("Argument expected! Please consult the help page!");
            exit(1);
        }
    }
}

fn help() {
    println!("[*] HELP - ret\n
        install - Installed and sets up all the tools\n
        update - Updates all the tools\n
        check - Checks for updates\n
        set-home - Sets the home directory where the tools will be installed\n
        -h/--help - prints out this");
    exit(0)
}

fn update() {
    
}

fn check() {
    
}

fn sethome(args: &Vec<String>) {
    
}

fn install() {
    let install_path: PathBuf = PathBuf::from("C:\\ret");

    match install_path.exists() {
        true => (),
        false => {
            std::fs::create_dir(install_path).unwrap();
        }
    }

    let ghidra_ver_file = std::fs::File::create(PathBuf::from("C:\\ret\\ghidra.ver")).unwrap();
    let die_ver_file = std::fs::File::create(PathBuf::from("C:\\ret\\die.ver")).unwrap();
    let x64dbg_ver_file = std::fs::File::create(PathBuf::from("C:\\ret\\x64dbg.ver")).unwrap();


}
