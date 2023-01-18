pub struct Solution;

impl Solution{
    pub fn longest_palindrome(s:String) -> i32{
        use std::collections::HashMap;

        let mut hm = HashMap::new();
        
        for char in s.chars(){
            *hm.entry(char).or_insert(0) += 1;
        }

        let mut max_length = 0;
        let mut has_odd = false;

        for value in hm.values() {
            if value % 2 == 0{
                max_length += value;
            }else {
                max_length += value - 1;
                has_odd = true;
            }
        }
        max_length + if has_odd {1} else {0}
    }
}
