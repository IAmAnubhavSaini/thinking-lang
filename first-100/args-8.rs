fn main() {
    if std::env::consts::OS == "linux" {
        println!("{:#?}", std::env::args());
    }
}
