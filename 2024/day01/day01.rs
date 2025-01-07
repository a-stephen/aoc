use std::fs;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let lines = input.lines().collect::<Vec<&str>>();

    for line in &lines {
        let rlitem = line.split_whitespace().collect::<Vec<_>>();
        left.push(rlitem[0].parse::<i32>().expect("Can not parse item!!"));
        right.push(rlitem[1].parse::<i32>().expect("Can not parse item!!"));
    }

    (left, right)
}

fn compute_difference(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    let mut sum: i32 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        let s: i32 = (l - r).abs();
        sum += s;
    }
    sum
}

fn compute_list_similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    for ritem in right {
        *right_map.entry(ritem).or_insert(0) += 1;
    }
    
    let iter_sum: i32 = left.iter().map(
        |x| x * right_map.get(x).unwrap_or(&0)
    ).sum();

    iter_sum
}

fn main() {
    let file_path = "./data/r_input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let (left, right) = parse_input(&input);
    let right_occurence = compute_list_similarity(left.clone(), right.clone());

    println!("{:?}", right_occurence);
}
