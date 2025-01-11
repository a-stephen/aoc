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
    let mut r = report.into_iter().collect::<Vec<&i32>>();
    for (idx, level) in r.clone().into_iter().enumerate() {
        if idx != 0 || &idx != &r.len() {
            r.remove(idx);
            println!("{:?}", r)
        }
        
    }
    
    true
}

//fn is_safe_dampened(report: &[i32]) -> bool {
//    let mut positive: bool = true;
//    let mut unsafe_count: usize = 0;
//    
//    let check_pos = |idx: usize, val: i32, other: i32| {
//        if idx == 0 {
//            let positive: bool = (other - val) > 0;
//            return true
//        } else if ((other - val) > 0) != positive {
//            return false;
//        }
//        true
//    };

//    let check_level = |val: i32, other: i32| {
//        let change: i32 = (other - val).abs();
//        (1..=3).contains(&change)
//    };

//    for (idx, val) in report.iter().enumerate() {
//        let other_idx: usize = idx + 1 + unsafe_count;
//        if other_idx >= report.len() {
//            break;
//        }

//        let other: i32 = report[other_idx];
//        let mut dampen: bool = false;

//        if !check_pos(idx, *val, other) {
//            dampen = true;
//        }

//        if !check_level(*val, other) {
//            dampen = true;
//        }

//        if dampen {
//            unsafe_count += 1;
//            if unsafe_count > 1 {
//                return false;
//            }

//            let other_idx: usize = idx + 1 + unsafe_count;
//            if other_idx >= report.len() {
//                break;
//            }
//            let other: i32 = report[other_idx];
//            if !check_pos(idx, *val, other) {
//                return false
//            }

//            if !check_level(*val, other) {
//                return false;
//            }
//        }

//    }
//    true
//}


fn compute_report_safety(reports: &Vec<Vec<i32>>) {
//    let part_a: usize = reports.iter().filter(|report| is_safe(&report)).count();
    let part_b: usize = reports.iter().filter(|report| is_safe_dampened_2(&report)).count();
//    println!("Part A: {:?}", part_a);
    println!("Part B: {:?}", part_b);
}

fn main() {
    let file_path = "./data/t_input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let reports = get_reports(&input);

    compute_report_safety(&reports);
}
