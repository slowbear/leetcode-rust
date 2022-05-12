pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut current = 1;
        for (index, elem) in nums.iter().enumerate() {
            current = current * elem;
            while left <= index && current >= k {
                current /= nums[left];
                left += 1;
            }
            // 注意要先+1，否则usize下溢
            res += index + 1 - left;
        }
        res as _
    }
}
