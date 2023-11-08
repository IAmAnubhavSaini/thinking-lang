fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut count: i32 = 0;
    for jewel in jewels.chars() {
        for stone in stones.chars() {
            if jewel == stone {
                count += 1;
            }
        }
    }
    return count;
}

fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut count: i32 = 0;
    for jewel in jewels.chars() {
        count += stones.chars().filter(|&x| x == jewel).count() as i32;
    }
    return count;
}

fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones.chars().filter(|&s| jewels.contains(s)).count() as i32
}
