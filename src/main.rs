use std::io::{Write, BufReader};
use std::env;
use std::process::{Command, Stdio};

extern crate termion;
use termion::{color, style};
use termion::input::TermRead;

fn main() {
    // Use the tty to prevent writing to stdout
    let mut tty = termion::get_tty().unwrap();

    // Collect the command from the input
    let args: Vec<String> = env::args().collect();
    let command: String = args.iter()
                              .skip(1)
                              .fold(String::new(), |acc, item| { acc + " " + item });

    // Confirm message
    writeln!(tty, "{}{}Are you sure? (y/n){}", color::Fg(color::Red), style::Bold, style::Reset).unwrap();
    tty.flush().unwrap();

    let mut reader = BufReader::new(tty);
    match TermRead::read_line(&mut reader) {
        Ok(answer) => {
            if answer == std::option::Option::Some("y".to_string()) {
                Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("Something went wrong");
            }
            else {
                writeln!(tty, "{}{}Okay, not running!{}", color::Fg(color::Red), style::Bold, style::Reset).unwrap();
            }
        }
        Err(error) => {
            //writeln!(tty, "{}{}Something went wrong: {}{}", color::Fg(color::Red), style::Bold, style::Reset, error).unwrap();
        }
    }
}

