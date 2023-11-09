use std::env;
use std::process::{Command};

fn main() {
    if env::consts::OS == "linux" {
        let output = Command::new("who").output();
        match output {
            Ok(value) => {
                let s = String::from_utf8(value.stdout);
                match s {
                    Ok(t) => println!("{t}"),
                    Err(x) => println!("ERROR: {}", x)
                }
            },
            Err(error) => println!("ERROR: {error}")
        }
    }
}

