// Given a list of integers, use a vector and return the mean (the average value), median (when
// sorted, the value in the middle position), and mode (the value that occurs most often; a hash
// map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let mut v = vec![9, 2, 1, 9, 2, 3, 1, 4, 1, 8];

    let mut sum = 0;

    for i in &v {
        sum += i;
    }

    let mean = sum / v.len();

    println!("mean is: {}", mean);

    v.sort();

    let middle_index = v.len() / 2 - 1;
    println!("median is: {}", &v[middle_index]);

    let mut map = HashMap::new();

    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut occurrence = 0;

    for (key, value) in map.iter() {
        if *value > occurrence {
            mode = **key;
            occurrence = *value;
        } 
    }

    println!("mode is: {}", mode);
}
