fn calculate_time(keyboard: String, word: String) -> i32 {
     let mut time = 0;
     let mut prev = 0;
     for (i, c) in word.chars().enumerate() {
         let curr = keyboard.chars().position(|x| x == c).unwrap();
         time +=  ((curr - prev) as i32).abs();
         prev = curr;
     }
     time
 }
