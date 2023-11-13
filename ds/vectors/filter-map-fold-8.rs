fn filter_map_fold(v: Vec<i32>, 
        ffilter: fn(i32) -> bool, 
        fmap: fn(i32) -> i32,
        init: i32,
        ffold: fn(i32, &i32) -> i32) -> i32 {
    let mut result = init;
    for (_, &v) in v.iter().enumerate() {
        if ffilter(v) {
            result = ffold(result, &fmap(v));
        }
    }
    result
}

fn even(v:i32) -> bool {
    v%2==0
}

fn sq(v:i32) -> i32 {
    v*v
}

fn add(x: i32, y: &i32) -> i32 {
    x+y
}

fn main() {
    let v:Vec<i32> = (0..10).collect();
    println!("{v:?} {:?}", filter_map_fold(v.clone(), even, sq, 0, add));
}

