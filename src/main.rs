
use std::fs::File;
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

fn cls() {
    std::process::Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();
}

fn exit_program() {
    std::process::exit(0);
}

fn whoami() -> io::Result<()> {
    let mut foke = File::open("name.txt")?;
    let mut conts = String::new();
    foke.read_to_string(&mut conts)?;
    println!("{conts}\n");
    Ok(())
}

fn signup() {
    cls();
    println!("Give me your new Username:");
    let mut newusern = String::new();
    io::stdin().read_line(&mut newusern).unwrap();
    let newusern = newusern.trim(); // Trim newline characters

    let mut userf = File::create("name.txt").expect("File creation failed");
    userf
        .write_all(newusern.as_bytes())
        .expect("Writing to file failed");

    println!("Signup complete! Returning to main menu.");
    thread::sleep(Duration::from_secs(2));
}

fn start() -> io::Result<()> {
    cls();
    println!("                                                                                  Welcome to HASNIX                                                                                             ");
    thread::sleep(Duration::from_secs(3));
    cls();
    println!("Volta HASNIX [Version 0.1]\n");
    println!("Volta Technologies. Free and Open-source for use GPLv3\n");

    let mut fptr = File::open("name.txt")?;
    let mut username = String::new();
    fptr.read_to_string(&mut username)?;
    username = username.trim().to_string();

    loop {
        println!("{username}::user->");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();

        match command {
            "clear_scr" => cls(),
            "quitos" => exit_program(),
            "whoami" => {
                if let Err(e) = whoami() {
                    eprintln!("Error executing whoami: {}", e);
                }
            }
            _ => println!("Unknown command: {}", command),
        }
    }
}

fn login() -> io::Result<()> {
    cls();

    println!("What is your username:");
    let mut usern = String::new();
    io::stdin().read_line(&mut usern).unwrap();
    let usern = usern.trim(); // Trim newline characters

    let mut f = File::open("name.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let contents = contents.trim(); // Trim newline characters

    if contents == usern {
        println!("Login successful!");
        start()?;
    } else {
        println!("Login failed! Returning to main menu.");
        thread::sleep(Duration::from_secs(2));
    }
    Ok(())
}

fn main() {
    loop {
        cls();
        println!("HASNIX: The OS of the Future\n");
        println!("Type 'login' to login or 'signup' to create a new account:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim(); // Trim newline characters

        match input {
            "login" => {
                if let Err(e) = login() {
                    eprintln!("Error during login: {}", e);
                    thread::sleep(Duration::from_secs(2));
                }
            }
            "signup" => signup(),
            _ => {
                println!("Invalid option, please choose 'login' or 'signup'");
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

