struct Solution;

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        // shadow mutable
        let mut nums = nums;

        // go until 1 number left
        while nums.len() != 1 {
            // create temp array
            let mut temp = nums.clone();

            // go up til current length -1 (as array shrinks by 1 every time)
            for i in 0..nums.len() - 1 {
                // add number and next
                temp[i] = (nums[i] + nums[i + 1]) % 10;
            }

            // shrink
            temp.pop();
            nums = temp;
        }

        // result will be last number standing
        *nums.first().unwrap()
    }
}
