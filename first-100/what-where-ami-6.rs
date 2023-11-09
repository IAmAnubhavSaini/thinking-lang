use std::{env};

fn main() {
    println!("{:#?}", env::current_dir());
    println!("{:#?}", env::current_exe());
}
