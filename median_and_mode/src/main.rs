// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
use std::collections::HashMap;

fn main() {
    let mut numbers = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 1);
    numbers.sort();
    let median = find_median(&numbers);
    let mode = find_mode(&numbers);

    println!("Median, {median}");
    println!("Mode, {mode:?}");
}

fn find_median(numbers: &Vec<i32>) -> f64{
    let length = numbers.len();
    let median: f64; 
    let middle_index = length/2;

    if length % 2 == 0 {
        // Even length: average the two middle elements
        median = (((numbers[middle_index - 1]) + (numbers[middle_index])) as f64) / 2.0
    } else {
        // Odd length: the middle element is the median
        median = numbers[middle_index] as f64
    }

    median
}

// find_mode uses a vector in the case of having more than one mode
fn find_mode(numbers: &Vec<i32>) -> Vec<i32>{
    let mut frequency = HashMap::new();
    let mut modes = Vec::new();

    for num in numbers {
        let count = frequency.entry(num).or_insert(0);
        *count += 1;
    }
    
    // println!("{frequency:?}"); // Printing a HashMap
    let highest_freq = *frequency.values().max().unwrap();
    
    for (key, value) in frequency.iter() {
        if *value == highest_freq {
            modes.push(**key);
        }
    }
    
    modes
}