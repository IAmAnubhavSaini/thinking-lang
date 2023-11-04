fn roman_to_int(s: String) -> i32 {
    let mut value = 0;
    for (i, v) in s.chars().enumerate() {
        match v {
            'M' => value += if i > 0 && s.chars().nth(i - 1) == Some('C') { 800 } else { 1000 },
            'D' => value += if i > 0 && s.chars().nth(i - 1) == Some('C') { 300 } else { 500 },
            'C' => value += if i > 0 && s.chars().nth(i - 1) == Some('X') { 80 } else { 100 },
            'L' => value += if i > 0 && s.chars().nth(i - 1) == Some('X') { 30 } else { 50 },
            'X' => value += if i > 0 && s.chars().nth(i - 1) == Some('I') { 8 } else { 10 },
            'V' => value += if i > 0 && s.chars().nth(i - 1) == Some('I') { 3 } else { 5 },
            _ => value += 1
        }
    }
    return value;
}

fn main() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("IV".to_string()), 4);
    assert_eq!(roman_to_int("IX".to_string()), 9);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}