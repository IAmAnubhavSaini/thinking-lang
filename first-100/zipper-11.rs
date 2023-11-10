fn squares(n: u64) -> Vec<u64> {
    (0..=n).map(|x| x*x).collect()
}

fn cubes(n: u64) -> Vec<u64> {
    (0..=n).map(|x| x*x*x).collect()
}

fn root_sq(n: u64) -> Vec<f64> {
    (0..=n).map(|x| f64::sqrt(x as f64)).collect()
}

fn root_cube(n: u64) -> Vec<f64> {
    (0..=n).map(|x| (x as f64).powf(1.0 / 3.0)).collect()
}

fn main() {
    let n = 256;
    let nums:Vec<u64> = (0..=n).collect();
    let sq = squares(n);
    let cu = cubes(n);
    let sq_r = root_sq(n);
    let cu_r = root_cube(n);
//    println!("{:?} {:?} {:?} {:?} {:?}", nums, sq, cu, sq_r(n), cu_r(n));

    let zipped:Vec<_> = nums.iter()
        .zip(sq.iter())
        .zip(cu.iter())
        .zip(sq_r.iter())
        .zip(cu_r.iter())
        .map(|((((&a, &b), &c), &d), &e)| (a, b, c, d, e))
        .collect();

    for &(a, b, c, d, e) in &zipped {
        println!("{a:8} {b:8} {c:8} {d:8.8} {e:8.8}");
    }
}

