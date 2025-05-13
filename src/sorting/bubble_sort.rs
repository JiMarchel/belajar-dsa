pub fn bubble_sort(nums: &mut [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                compare = true;
                nums.swap(i, i + 1);
            }
        }

        len -= 1;
    }
}
