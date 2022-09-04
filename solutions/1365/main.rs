impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // create vector for all nums (for counts) due to constraint 0 <= nums[i] <= 100
        let mut counts: Vec<i32> = vec![0; 101];

        // count nums
        for num in nums.iter() {
            counts[*num as usize] += 1;
        }

        // for each num, amount of numbers smaller or equal than it is running sum
        for index in 1..101 {
            counts[index] += counts[index - 1]
        }

        // each number's smaller than count is sum of all numbers to left
        nums.iter()
            .map(|num| match *num != 0 {
                true => counts[*num as usize - 1],
                false => 0,
            })
            .collect()
    }
}
