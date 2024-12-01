use std::fs;

fn main() {
    // Reading the file
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
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

    // Sorting the values
    left.sort();
    right.sort();

    // Calculating the total

    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }
    println!("The total is: {}", total);
}
