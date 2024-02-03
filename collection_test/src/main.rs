use std::collections::HashMap;

fn get_median_mode(data: &Vec<i32>) -> (i32, i32) {
    let mut sorted = data.clone();

    sorted.sort();
    let median = sorted.get(sorted.len() / 2).unwrap();

    let mut map = HashMap::new();

    for val in sorted.iter() {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // find key has largest value in map
    let mode = map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| **k)
        .unwrap();

    (*median, mode)
}
fn main() {
    let integer_list = vec![1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 33, 33, 33];

    let (median, mode) = get_median_mode(&integer_list);

    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
