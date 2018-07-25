use std::io::{self, Write};
use std::env;
use std::process::{Command, Stdio};

extern crate termion;

fn main() {
    // Use the tty to prevent writing to stdout
    let mut tty = termion::get_tty().unwrap();

    // Collect the command from the input
    let args: Vec<String> = env::args().collect();
    let command: String = args.iter()
                              .skip(1)
                              .fold(String::new(), |acc, item| { acc + " " + item });

    // Confirm message
    writeln!(tty, "Are you sure? (y/n)").unwrap();
    tty.flush().unwrap();

    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer == "y\n" {
                Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("Something went wrong");
            }
            else {
                println!("Okay, not running!")
            }
        }
        Err(error) => println!("Something went wrong: {}", error),
    }
}

