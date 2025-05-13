pub fn shell_sort(nums: &mut [i32]) {
    // Internal function for insertion sort
    // with elements exchanged distance is gap
    fn ist_sort(nums: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;

        while i < nums.len() {
            let mut pos = i;
            let curr = nums[pos];

            while pos >= gap && curr < nums[pos - gap] {
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }

            nums[pos] = curr;
            i += gap;
        }
    }

    let mut gap = nums.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}
