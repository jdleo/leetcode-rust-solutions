struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        // to count "visits"
        let mut visits: HashMap<String, i32> = HashMap::new();

        // go through each cpdomain
        for cpdomain in cpdomains.into_iter() {
            let cpdomain_split: Vec<String> = cpdomain.split(" ").map(String::from).collect();
            let (visit_count, domain) = (
                cpdomain_split[0].parse::<i32>().unwrap(),
                cpdomain_split[1].to_string(),
            );

            // find all subdomains too
            let subdomains: Vec<String> = domain.split(".").map(String::from).collect();
            for i in 0..subdomains.len() {
                let subdomain = &subdomains[i..].join(".");
                *visits.entry(subdomain.to_string()).or_insert(0) += visit_count;
            }
        }

        // map each key/pair to expected formatted string
        visits
            .into_iter()
            .map(|(domain, visit_count)| format!("{} {}", visit_count, domain))
            .collect()
    }
}
