fn reverse_vowels(s: String) -> String {
    let mut vowels:Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => vowels.push(c),
            _ => continue
        }
    }

    let mut ri = vowels.len() - 1;
    let mut t = String::new();

    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                t.push(vowels[ri]);
                ri -= 1;
            },
            _ => t.push(c)
        }
    }
    return t;
}
