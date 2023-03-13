pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    let mut bitwise_and = left;
    let powers_of_2 = bin(left);
    for (index, has_power) in powers_of_2.iter().enumerate() {
        if !*has_power {
            continue;
        }
        
        if contained_in_range(left, right, bitwise_and + 2_i32.pow(index as u32)) {
            bitwise_and &= left + 2_i32.pow(index as u32);
        }
    }
    return bitwise_and;
}

fn contained_in_range(left: i32, right: i32, num: i32) -> bool {
    return left <= num && num <= right;
}

fn bin(mut num: i32) -> Vec<bool>{
    let mut result = Vec::new();
    while num > 0 {
        result.push((num & 1) == 1);
        num = num >> 1;
    }
    return result;
}