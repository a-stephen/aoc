use std::fs;
// use std::ops::Index;

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

fn is_safe(report: &Vec<i32>) -> bool {
    // All decreasing or all increasing
    // let mut unsafe_count: usize = 0;
    let diff: Vec<i32> = report.iter().zip(report.iter().skip(1)).map(|(a, b)| a - b).collect::<Vec<i32>>();
    let positive: bool = diff[0] > 0;
    //unsafe_count += diff.iter().filter(|x: &&i32| (**x > 0) != positive).count();
    // if unsafe_count > unsafe_allowed {
    //    return false;
    // }
    let direction: bool = diff.iter().all(|x: &i32| (*x > 0) == positive );

    if !direction {
        return false;
    }
    let level_change: bool = diff.iter().map(|x| x.abs()).all(|x| (1..=3).contains(&x));

    //unsafe_count += diff.iter().map(|x| x.abs()).filter(|x| !(1..=3).contains(x)).count();

    //unsafe_count < unsafe_allowed
    level_change
}

fn is_safe_dampened_2(report: &Vec<i32>) -> bool {
    let mut split_report: Vec<Vec<i32>> = vec![];
    let report_clone: Vec<i32> = report.into_iter().cloned().collect();
    for (idx, _level) in report_clone.clone().into_iter().enumerate() {
        let arr_right: &[i32] = &report_clone[0..idx];
        let arr_left: &[i32] = &report_clone[idx+1..];
        let report_one_level_out = [arr_right, arr_left].concat();
        split_report.push(
            report_one_level_out.into_iter().collect::<Vec<i32>>()
        );
    }
    let dampened_report: usize = split_report.iter().filter(
        |s_report| is_safe(&s_report)
    ).count();

    let safe_one_level_out: bool = dampened_report >= 1;

    safe_one_level_out
}


fn compute_report_safety(reports: &Vec<Vec<i32>>) {
    let part_a: usize = reports.iter().filter(|report| is_safe(&report)).count();
    let part_b: usize = reports.iter().filter(|report| is_safe_dampened_2(&report)).count();
    println!("Part A: {:?}", part_a);
    println!("Part B: {:?}", part_b);
}

fn main() {
    let file_path = "./data/r_input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let reports = get_reports(&input);

    compute_report_safety(&reports);
}
