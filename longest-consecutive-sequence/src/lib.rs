use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut nums_set = HashSet::new();
    for n in nums.iter() {
        nums_set.insert(*n);
    }

    let mut longest_length = 1;
    for n in nums.iter() {

        if nums_set.contains(&(n + 1)) {
            continue;
        }

        let mut current_length = 1;
        let mut next_in_sequence = n - 1;
        loop {
            if nums_set.contains(&next_in_sequence) {
                current_length += 1;
                if current_length > longest_length {
                    longest_length = current_length;
                }
                next_in_sequence -= 1;
            } else {
                break;
            }
        }
    }

    return longest_length;
}