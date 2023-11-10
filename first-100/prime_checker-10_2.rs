use std::collections::HashSet;

fn is_prime(n: u32, primes: &HashSet<u32>) -> bool {
    if n < 2 {
        return false;
    }

    for &p in primes {
        if n%p == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let mut primes:HashSet<u32> = HashSet::new();

    for i in 0..=1000000 {
        if is_prime(i, &primes) {
            primes.insert(i);
            print!(" {i:8} ");
        }
    }

    println!("Number of primes found: {}", primes.len());
}

/*
100000
real    0m2.697s
user    0m2.696s
sys     0m0.000s

1000000
real    3m0.332s
user    3m0.327s
sys     0m0.004s

*/
