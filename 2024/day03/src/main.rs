use regex::Regex;
use std::fs;

fn read_input(file_path: &str) -> String {
    let instruction_string = fs::read_to_string(file_path).unwrap();
    return instruction_string
}

fn _parse_instruction(intrustion_line: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    for (_, [left, right]) in re.captures_iter(intrustion_line).map(|c| c.extract()) {
        results.push((left.parse().unwrap(), right.parse().unwrap()));
    }
    results
}

fn do_parse_instruction(intrustion_line: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d*),(\d*)\)|don't|do").unwrap();
    let mut results: Vec<(i32, i32)> = vec![];
    let mut do_next = true;
    let mut sum = 0;
    for capture in re.captures_iter(intrustion_line) {
        let proceed_instruct: &str = capture.get(0).map_or("", |m| m.as_str());
        match proceed_instruct {
            "do" => do_next = true,
            "don't" => do_next = false,
            mul if do_next => {
                let left: i32 = capture.get(1).map_or(0, |m| m.as_str().parse().unwrap());
                let right: i32 = capture.get(2).map_or(0, |m| m.as_str().parse().unwrap());
                results.push((left, right));
            },
            _ => {},
        }
        
    }
    results
}

fn main() {
    let test = read_input("src/r_input.txt");
    let results: Vec<(i32, i32)> = do_parse_instruction(&test);
    let res_sum = results.into_iter().map(|item| item.0 * item.1).collect::<Vec<i32>>();
    println!("{:?}", res_sum.into_iter().sum::<i32>())
}
