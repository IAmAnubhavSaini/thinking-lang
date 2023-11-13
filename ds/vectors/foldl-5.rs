fn foldler(v: Vec<i32>, init: i32, f: fn(i32, i32) -> i32) -> i32 {
    v.iter().fold(init, |a, &c| f(a, c))
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let v:Vec<i32> = (0..10).collect();
    println!("{v:?} {:?}", foldler(v.clone(), 0, add));
}

