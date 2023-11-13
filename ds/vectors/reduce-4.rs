fn reducer<F>(v: Vec<i32>, reduce_fn: F) -> i32 
where 
F: Fn(i32, i32) -> i32
{
    let result = v.iter().cloned().reduce(|a, c| reduce_fn(a, c));
    match result {
        Some(v) => v,
        None => 0
    }
}

fn add(x: i32, y: i32) -> i32 {
    x+y
}

fn main() {
    let v:Vec<i32> = (0..10).collect();
    println!("{v:?} {}", reducer(v.clone(), add));
}

