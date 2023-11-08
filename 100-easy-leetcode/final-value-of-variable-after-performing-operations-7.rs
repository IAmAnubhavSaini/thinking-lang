fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut value = 0;
    for s in operations.iter() {
        match s.contains("++") {
            true => value += 1,
            false => value -= 1
        }
    }
    value
}

fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().map(|s| if s.contains("++") { 1 } else { -1 }).sum::<i32>()
}

fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |a, c| if c.contains("++") { a+1 } else { a-1 })
}
