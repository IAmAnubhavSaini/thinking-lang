fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            if target == n + m {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![-1 as i32, -1 as i32];
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::new();

    for (i, v) in nums.iter().enumerate() {
        // check if v exists in diffs
        if diffs.iter().any(|&x| &x == v) {
            // if it does, return [index of v, i]
            let h = diffs.iter().position(|&x| &x == v);
            match h {
                Some(inner) => return vec![inner as i32, i as i32],
                None => continue
            }
        }
        // if it doesn't, add target-v to diffs
        let diff = target - v;
        diffs.push(diff);
    }
    return vec![-1 as i32, -1 as i32];
}
