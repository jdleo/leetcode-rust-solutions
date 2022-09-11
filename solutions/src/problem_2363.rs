struct Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let (mut items1, mut items2) = (items1, items2);

        // first, sort each items by id
        items1.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        items2.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        // pointers for items1 and 2
        let (mut i, mut j) = (0usize, 0usize);

        // go until the end
        while i < items1.len() && j < items2.len() {
            // check if equal id's
            if items1[i][0] == items2[j][0] {
                // merge
                result.push(Vec::from([items1[i][0], items1[i][1] + items2[j][1]]));
                i += 1;
                j += 1;
            } else if items1[i][0] < items2[j][0] {
                result.push(items1[i].clone());
                i += 1;
            } else {
                result.push(items2[j].clone());
                j += 1;
            }
        }

        // if anything still has length left, just add it to the end
        while i < items1.len() {
            result.push(items1[i].clone());
            i += 1;
        }
        while j < items2.len() {
            result.push(items2[j].clone());
            j += 1;
        }

        result
    }
}
