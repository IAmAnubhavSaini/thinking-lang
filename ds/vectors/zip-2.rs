fn zipper(n: i32) -> Vec<(i32, i32)> {
    let f_n:Vec<i32> = (0..n).collect();
    let sqn: Vec<i32> = (0..n).map(|x| x*x).collect();
    let zipped:Vec<_> = f_n.iter().zip(sqn.iter())
        .map(|(&a, &b)| (a, b))
        .collect();
    
    zipped
}

fn main() {
    let z = zipper(10);
    println!("{z:?}");
}

