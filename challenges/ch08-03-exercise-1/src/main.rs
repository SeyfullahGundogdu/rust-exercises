use std::collections::HashMap;
use rand::Rng;
fn main() {
    println!("Initializing array of zeros and filling it with values between 0-20 randomly.");

    // fill the array with random numbers between 0 (inclusive) and 21 (exclusive)
    let mut t = [0;100];
    for _elem in &mut t {
        *_elem = rand::thread_rng().gen_range(0..21);
    }
    println!("Array mean: {}",array_mean(&t));
    println!("Array median: {}", array_median(&t));
    println!("Array mode: {:?}", array_mode(&t));
    
}
///calculates mean of array by adding elements and dividing the result to `array.len()`
/// 
/// [Arguments]
/// * `array` - the array of integers to be processed.
/// 
/// [Returns]
/// * 0 if the array is empty, `average` of elements inside `array` otherwise.
fn array_mean(array: &[i32]) -> f32 {
    if let Some(_u32) = Some(array.len() - 1) {
        array.iter().sum::<i32>() as f32 / array.len() as f32
    } else {
        0.0
    }
    
}

fn array_median(array: &[i32]) -> f32 {
    //create dummy array and copy contents of original array
    let mut temp = vec![0;array.len()];
    temp[..].clone_from_slice(array);
    //sort the dummy array
    temp.sort();
    // if array has even number of elements, median is going to be mean of elements in the middle
    //so we can call arr_mean function inside for simplicity
    let len = temp.len();
    match len % 2 {
        1 => temp[len / 2] as f32,
        //slices are start inclusive-end exclusive, therefore we get the slice[middle - 1, middle + 1];
        0 => array_mean(&temp[(len / 2) - 1..(len / 2) +1]),
        _ => panic!("array length is not an integer!!!")
    }
    
}

fn array_mode(array: &[i32]) -> (Vec<i32>,i32) {
    //create a mutable HashMap to store values' occuring times.
    let mut dummy_map = HashMap::new();
    for elem in array {
        let stat = dummy_map.entry(elem).or_insert(0);
        *stat += 1;
    }
    //iterate through the map and calculate the max number of occurences.
    let mut max = i32::MIN;
    for (_, value) in &dummy_map {
        if *value > max {
            max = *value;
        }
    }

    //create vector and append items that occurs the most to it.
    let mut vec: Vec<i32> = Vec::new();
    for (key, value) in dummy_map {
        if value == max {
            vec.push(*key);
        }
    }
    
    (vec,max)
}
