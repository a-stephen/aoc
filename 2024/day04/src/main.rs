// use std::io::Read;

fn parse_input() -> Result<Vec<Vec<char>>, ()> {
    // let mut text = String::new();
    let text = std::fs::read_to_string("puzzle/test_input.txt").map_err(
        |err| {
            eprintln!("ERROR: Reporting failure to open file: {err}")
        }
    )?;
    // input_test.read_to_string(&mut text);

    let input_char: Vec<Vec<char>> = text.lines().map(
        |line| {
            line.chars().collect::<Vec<char>>()
        }
    ).collect::<Vec<Vec<char>>>();
    Ok(input_char)
}



fn main() {
    let input_char = parse_input().unwrap();
    // let mut count = 0;
    for (x_index, x_vec) in input_char.iter().enumerate() {
        for (y_index, y_char) in x_vec.iter().enumerate() {
            println!("{x_index}; {y_index}")
            // if *y_char == 'X' {
            //     // println!("Char found at {y_index} is {y_char}")
            //     count += 1;
            // }
        }
    }

    // println!("Count all occurence of X => {count}")
}
