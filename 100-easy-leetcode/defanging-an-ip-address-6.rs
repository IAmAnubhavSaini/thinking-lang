fn defang_i_paddr(address: String) -> String {
    let mut defangled = String::new();
    for c in address.chars() {
        match c {
            '.' => defangled.push_str("[.]"),
            _ => defangled.push(c)
        }
    }
    defangled
}
