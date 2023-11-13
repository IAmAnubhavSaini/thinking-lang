fn filterer(v: Vec<i32>, f: fn(i32) -> bool) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for (_, &v) in v.iter().enumerate() {
        if f(v) {
            out.push(v);
        }
    }
    out
}

fn even(v: i32) -> bool {
    v % 2 == 0
}

fn main() {
    let v:Vec<i32> = (0..10).collect();
    println!("{v:?} {:?}", filterer(v.clone(), even));
}

