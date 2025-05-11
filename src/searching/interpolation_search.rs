pub fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut low = 0usize;
    let mut high = nums.len() - 1;

    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        //in this case target isn't in the nums
        if high <= low || target < low_val || target > high_val {
            return false;
        }

        // calculate the position for insertion
        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let interpolant = low + offset as usize;

        //update high and low
        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            // found: nums[interpolant] == target
            return true;
        }
    }
}
