use crate::utils::file_reader;

pub fn solve_part_one() {

    let mut result: u32 = 0;
    let path_string = "/inputs/day_one.txt";

    if let Ok(lines) = file_reader::read_lines(path_string) {    
        // Uses the iterator to return each optional line
        for line in lines {
            if let Ok(read_line) = line {
                result = result + get_numeric_values(read_line);
            }
        }
    }

    print!("\nAnswer for Day 1, Part 1 is {0}", result);
}

fn get_numeric_values(line: String) -> u32 {
    const RADIX: u32 = 10;
    let numeric_chars: Vec<Option<u32>> = line.chars().map(|c: char| c.to_digit(RADIX)).collect();
    let mut numeric_vals = Vec::new();

    for i in &numeric_chars {
        match i {
            Some(i) => numeric_vals.push(i),
            _ => (),
        }
    }
    
    let mut first_element = numeric_vals.first().cloned().unwrap().to_string();
    let last_element = numeric_vals.last().cloned().unwrap().to_string();
    first_element.push_str(&last_element);
    first_element.parse::<u32>().unwrap()
}
