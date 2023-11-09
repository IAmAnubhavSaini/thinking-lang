

fn fib_cached(n: i32, cache: Vec<i32>) -> i32 {
    if n < 1 { return 0; }
    let len = cache.len() as i32;
    if n < len - 1 {
        return cache[n as usize];
    }
    let a = fib_cached(n-2, cache);
    let b = fib_cached(n-1, cache);
    cache.push(a+b);
    return cache[n as usize];
}

fn fib(n: i32) -> i32 {
    if n < 0 { return 0; }
    if n == 0 || n == 1 { return n; }
    return fib(n-1) + fib(n-2);
}

fn fib2(n: i32) -> i32 {
    if n < 0 { return 0; }
    if n == 0 || n == 1 { return n; }
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = b + tmp;
    }
    return a;
}
fn main() {
    println!("{}", fib(10));
    println!("{}", fib2(10));
    let cache: Vec<i32> = vec![0, 1];
    println!("{}", fib_cached(10, cache));
}
