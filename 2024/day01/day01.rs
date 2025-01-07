use std::fs;

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

fn main() {
    let file_path = "./data/r_input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let (left, right) = parse_input(&input);

    let sum = compute_difference(left.clone(), right.clone());
    
    println!("{:?}", sum);
    // println!("{:?}", parsed_input)
    // let (mut left: Vec<i64>, mut right: Vec<i64>) = input.lines().map(
    //     |x: &str| {
    //         let pairs: Vec<&str> = x.split("   ").collect::<Vec<&str>>();
    //         (pairs[0].parse::<i64>().unwrap(), pairs[1].parse::<i64>().unwrap())
    //     }
    // ).collect::<{Vec<i64>, Vec<i64>}>();
}
