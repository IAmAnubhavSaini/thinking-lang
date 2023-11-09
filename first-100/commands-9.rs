fn main() {
    if std::env::consts::OS == "linux" {
        let args:Vec<String> = std::env::args().collect();
        for (_, arg) in args[1..].iter().enumerate() {
            let output = std::process::Command::new(arg).output();
            println!("Command: {arg}");
            //println!("Command: {arg}\n{:#?}", output);
            match output {
                Ok(x) => {
                    let stdout = String::from_utf8(x.stdout);
                    match stdout {
                        Ok(value) => println!("{value}"),
                        Err(error) => println!("ERROR: {error}")
                    }
                },
                Err(error) => println!("ERROR: {error:#?}")
            }
        }
    }
}

