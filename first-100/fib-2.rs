fn fib_upto(upto:i16) {
    if upto < 1 {
        return;
    }
    let mut a = 0;
    let mut b = 1;
    while a <= upto {
        print!("{:8}\n", a);
        let tmp = a;
        a = b;
        b = tmp + a;
    }
}

fn fib_n(n: i16) {
    if n < 1 {
        return;
    }
    let mut a = 0;
    let mut b = 1;
    let mut count = 0;
    while count < n {
        print!("{:8} {:8}\n", count + 1, a);
        let temp = a;
        a = b;
        b = temp + a;
        count += 1;
    }
}

fn main() {
    print!("Upto 10\n");
    fib_upto(10);
    print!("Upto 100\n");
    fib_upto(100);
    print!("First 10 terms\n");
    fib_n(10);
    print!("First 20 terms\n");
    fib_n(20);
}

