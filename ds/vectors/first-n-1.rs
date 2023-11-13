fn first_n(n: i32) -> Vec<i32> {
    (0..n).collect()
}

fn main() {
    let f10 = first_n(10);
    println!("{f10:?}");
}

