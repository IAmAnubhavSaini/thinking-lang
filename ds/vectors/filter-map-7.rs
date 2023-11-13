fn filter_map(v: Vec<i32>, ffilter: fn(i32) -> bool, fmap: fn(i32) -> i32) -> Vec<i32> {
    let mut out:Vec<i32> = Vec::new();
    for (_, &v) in v.iter().enumerate() {
        if ffilter(v) {
            out.push(fmap(v));
        }
    }
    out
}

fn even(v:i32) -> bool {
    v%2==0
}

fn sq(v:i32) -> i32 {
    v*v
}

fn main() {
    let v:Vec<i32> = (0..10).collect();
    println!("{v:?} {:?}", filter_map(v.clone(), even, sq));
}

