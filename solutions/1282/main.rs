use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        // hash map to store groups of groups
        let mut groups: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

        // go through people
        for (person, group_size) in group_sizes.into_iter().enumerate() {
            // get the group of groups
            let group = groups.entry(group_size).or_insert(Vec::new());

            // check if this group has any groups
            if let Some(current_group) = group.last() {
                // check if this group is already at max capacity
                if current_group.len() as i32 == group_size {
                    // push empty group to group
                    group.push(Vec::new());
                }

                // push this person to last group
                group.last_mut().unwrap().push(person as i32);
            } else {
                // no group exists yet, so just create with this person
                group.push(Vec::from([person as i32]));
            }
        }

        // result of groups
        let mut result: Vec<Vec<i32>> = Vec::new();
        for group_bucket in groups.values() {
            for group in group_bucket {
                result.push(group.to_vec());
            }
        }

        result
    }
}
