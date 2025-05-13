pub fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low > high {
        return;
    }

    // left marker and right marker
    let mut lm = low;
    let mut rm = high;
    while lm < rm {
        // right marker move to left gradually
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }

        // left marker move to right gradually
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }

        //exchange data between position lm and rm
        nums.swap(lm, rm);
    }

    nums.swap(low, lm);

    if lm > 1 {
        quick_sort(nums, low, lm - 1);
    }

    quick_sort(nums, rm, 1 + high);
}
