fn check_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n/2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    for i in 0..=1000000 {
        if check_prime(i) {
            print!(" {i:8} ");
        }
    } 
    println!("");
}

/*
100000
real    0m1.056s
user    0m1.056s
sys     0m0.000s

1000000
real    1m26.533s
user    1m26.525s
sys     0m0.008s

*/

