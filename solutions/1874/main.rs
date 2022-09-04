impl Solution {
    pub fn min_product_sum(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        // sort array 1 ascending, array 2 descending
        nums1.sort_by(|a, b| a.cmp(b));
        nums2.sort_by(|a, b| b.cmp(a));

        // smallest with largest will be minimum sum
        // so zip both arrays, sum the products
        nums1
            .iter()
            .zip(nums2.iter())
            .fold(0, |acc, (a, b)| acc + a * b)
    }
}
