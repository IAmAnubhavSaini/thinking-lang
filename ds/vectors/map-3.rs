fn mapper(vec: &Vec<i32>, f_n: fn(i32) -> i32) -> Vec<i32> {
    vec.iter().map(|x| f_n(*x)).collect()
}

fn add1(x: i32) -> i32 {
    x+1
}

fn main() {
    let v:Vec<i32> = (0..10).map(|x| x).collect();
    println!("{v:?} {:?}", mapper(&v, add1));
}

