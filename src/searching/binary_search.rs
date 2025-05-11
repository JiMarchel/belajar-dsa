pub fn binary_search(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high {
        let mid = (low + high) >> 1;
        //low and high may cause overflow, use substraction
        // let mid: usize = low + ((high - low) >> 1)

        if num == nums[mid] {
            return true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    false
}
