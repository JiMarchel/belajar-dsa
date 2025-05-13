pub fn insertion_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];
        while pos > 0 && curr < nums[pos - 1] {
            //move element to right
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }

        //insert element:curr
        nums[pos] = curr;
    }
}
