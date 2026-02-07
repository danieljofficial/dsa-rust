use std::collections::HashSet;

fn main() {
    fn longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut max_length = 0;
        let mut char_set = HashSet::new();

        for right in 0..chars.len() {
            while char_set.contains(&chars[right]) {
                char_set.remove(&chars[left]);
                left += 1;
            }

            char_set.insert(&chars[right]);
            max_length = max_length.max(right - left + 1);
        }

        max_length as i32
    }

    let result = longest_substring(String::from("pwwkew"));
    println!("{:#?}", result)
}
