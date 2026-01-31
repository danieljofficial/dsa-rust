use std::collections::HashMap;

fn main() {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen_numbers = HashMap::new();

        for (position, &current_num) in nums.iter().enumerate() {
            let needed_num = target - current_num;

            if let Some(&needed_position) = seen_numbers.get(&needed_num) {
                return vec![needed_position as i32, position as i32];
            }

            seen_numbers.insert(current_num, position);
        }

        vec![]
    }
    let result = two_sum(vec![1, 2, 3, 4], 6);
    println!("{:?}", result)
}
