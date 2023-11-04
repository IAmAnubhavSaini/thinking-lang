fn is_palindrome_1(x: i32) -> bool {
    let mut c = if x < 0 { -x } else { x };
    let mut n = 0;
    while c > 0 {
        n = n * 10 + (c % 10);
        c /= 10;
        // print!("c:{} n:{}\n", c, n);
    }
    return n == x;
}

fn is_palindrome_2(x: i32) -> bool {
    if x < 0 { return false; }
    let xs = x.to_string();
    let mut i = 0;
    let mut j = xs.len() - 1;
    while i < j {
        if xs.chars().nth(i) != xs.chars().nth(j) {
            return false;
        }
        i += 1;
        j -= 1;
    }
    return true;
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 { return false; }
    let xs: String = x.to_string();
    let rxs: String = xs.chars().rev().collect();
    return xs == rxs;
}


fn main() {
    println!("{}", is_palindrome(121));
    println!("{}", is_palindrome(-121));
    println!("{}", is_palindrome(10));
    println!("{}", is_palindrome(-101));
}