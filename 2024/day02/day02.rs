use std::fs;

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = vec![];
    let lines = input.lines().collect::<Vec<&str>>();
    // let reports = lines.iter().for_each(|x| { x.split(" "); }).collect::<Vec<&str>>();
    //
    for line in &lines {
        let report = line.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        reports.push(report)
    }
    reports
}

fn main() {
    let file_path = "./data/t_input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let reports = parse_input(&input);

    println!("{:?}", reports);
}
