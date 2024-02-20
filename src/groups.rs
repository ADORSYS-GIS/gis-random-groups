// src/groups.rs

use std::collections::HashMap;
use std::error::Error;

use csv::ReaderBuilder;
use rand::seq::SliceRandom;

pub fn generate_groups(file_path: &str, n_groups: usize) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    // Read the CSV file and collect the records into a Vec<(String, String)>
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;

    // The `deserialize` method returns an iterator of the internal record iterator
    let records = rdr
        .deserialize()
        .map(|result| result.expect("a CSV record"))
        .collect::<Vec<(String, String)>>();

    // Group
    let mut groups_by_gender: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over the records and group them
    for (name, gender) in records {
        groups_by_gender.entry(gender).or_default().push(name);
    }

    // Shuffle the names and distribute them into the groups
    let mut rng = rand::thread_rng();

    // Create a vector of empty vectors
    let mut groups: Vec<Vec<String>> = vec![Vec::new(); n_groups];

    // Iterate over the groups and shuffle the names
    for (_gender, mut names) in groups_by_gender {
        names.shuffle(&mut rng);
        for (i, name) in names.into_iter().enumerate() {
            groups[i % n_groups].push(name);
        }
    }

    Ok(groups)
}
