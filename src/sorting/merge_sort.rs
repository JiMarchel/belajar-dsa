pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]);
        merge_sort(&mut nums[mid..]);
        merge(nums, mid);
    }
}

pub fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0;
    let mut k = mid;
    let mut temp = Vec::new();

    for _j in 0..nums.len() {
        if k == nums.len() || i == mid {
            break;
        }

        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    // to make sure all data been solved
    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            temp.push(nums[j]);
        }
    }

    // put temp data back to nums, finish sort
    for j in 0..nums.len() {
        nums[j] = temp[j];
    }
}
