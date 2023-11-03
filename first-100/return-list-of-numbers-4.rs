fn list_upto(n:i16) -> Vec<i16> {
    let mut v: Vec<i16> = Vec::new();
    let mut i = 0;
    while i < n {
        v.push(i);
        i += 1;
    }
    return v;
}

fn main() {
    let list = list_upto(10);
    print!("List upto 10:\n{:#?}\n", list);
    let list2 = list_upto(100);
    print!("List upto 100:\n{:?}\n", list2);
}
