fn remove_vowels(s: String) -> String {
    let mut t = String::new();
    for ch in s.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => continue,
            _ => t.push(ch)
        }
    }
    t
}
