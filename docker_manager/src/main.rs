use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help();
        exit(0);
    }

    match args[1].trim() {
        "install" => install(),
        "uninstall" => uninstall(),
        "update" => update(),
        "start" => start(),
        "stop" => stop(),
        "help" => help(),
        _ => {
            println!("Command not found!");
            exit(1);
        }
    }
}

fn help() {
    println!("Docker Manager for Windows
Commands:\n
    [*] install - Installs Docker in WSL, downloads all the binaries and adds them to path\n
    [*] uninstall - Uninstalls Docker and removes the wsl distribution\n
    [*] update - Updates docker and binaries\n
    [*] start - Starts the WSL machine and the deamon\n
    [*] stop - Stops the WSL machine and the deamon\n
    [*] help - Shows this help message\n");
}

fn install() {
    Command::new("wsl")
        .arg("--install")
        .arg("-d")
        .arg("Debian")
        .status()
        .expect("Unable to create the WSL machine!");
}

fn uninstall() {

}

fn update() {

}

fn start() {

}

fn stop() {

}
