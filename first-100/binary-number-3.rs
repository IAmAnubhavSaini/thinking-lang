fn print_number_in_binary(n: i16) {
    print!("{:8} {:8b}\n", n, n);
}

fn main() {
    let mut i = 0;
    while i < 10 {
        print_number_in_binary(i);
        i += 1;
    }
}

