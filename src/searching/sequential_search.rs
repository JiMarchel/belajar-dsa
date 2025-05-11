pub fn sequential_search(nums: &[i32], num: i32) -> bool {
    for n in nums {
        if *n == num {
            return true;
        }
    }

    false
}
