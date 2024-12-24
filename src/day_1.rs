use std::collections::HashMap;
use std::fs;

pub fn part1(file: &str) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut total = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let mut numbers = line.split_whitespace();
        let left_num = numbers.next().unwrap().parse::<i32>().unwrap();
        let right_num = numbers.next().unwrap().parse::<i32>().unwrap();
        left.push(left_num);
        right.push(right_num);
    }

    if left.len() != right.len() {
        panic!("The number of left and right values are not equal");
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }

    println!("The answer is: {}", total);
}

pub fn part2(file: &str) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    // Initializing the variables
    let mut total = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    // Parsing the file
    for line in contents.lines() {
        let mut numbers = line.split_whitespace();
        let left_num = numbers.next().unwrap().parse::<i32>().unwrap();
        let right_num = numbers.next().unwrap().parse::<i32>().unwrap();
        left.push(left_num);
        right.push(right_num);
    }

    if left.len() != right.len() {
        panic!("The number of left and right values are not equal");
    }
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in right {
        *map.entry(num).or_insert(0) += 1;
    }

    for num in left {
        match map.get(&num) {
            Some(&count) => {
                total += count * num;
            }
            None => {}
        }
    }

    println!("The answer is: {}", total);
}
