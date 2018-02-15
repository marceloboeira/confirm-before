use std::io;
use std::env;
use std::process::Command;

fn main() {
    // Collect the command from the input
    let args: Vec<String> = env::args().collect();
    let command: String = args.iter()
                              .skip(1)
                              .fold(String::new(), |acc, item| { acc + " " + item });

    // Confirm
    println!("Are you sure? (y/n)");

    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer == "y\n" {
                Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .spawn()
                    .expect("foo");
            }
            else {
                println!("Okay, not running!")
            }
        }
        Err(error) => println!("Something went wrong: {}", error),
    }
}

