struct Solution;

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        // biggest and smallest
        let (mut first_biggest, mut second_biggest, mut first_smallest, mut second_smallest) =
            (i32::MIN, i32::MIN, i32::MAX, i32::MAX);

        // go through nums to get biggest/smallest
        for num in nums.into_iter() {
            if num > first_biggest {
                second_biggest = first_biggest;
                first_biggest = num;
            } else if num > second_biggest {
                second_biggest = num
            }

            if num < first_smallest {
                second_smallest = first_smallest;
                first_smallest = num;
            } else if num < second_smallest {
                second_smallest = num
            }
        }

        // max product diff is two largest - two smallest
        first_biggest * second_biggest - first_smallest * second_smallest
    }
}
