use std::fs::read_to_string;

fn main() {
    let input_file = "input_1.txt"; //"input_1.txt";
    part_one(input_file);
    part_two(input_file);
}

fn part_one(path: &str) {
    let mut sum = 0;
    for i in read_to_string(path).unwrap().lines() {
        let first_digit = get_first_digit(i);
        let last_digit = get_last_digit(i);
        if first_digit.is_none() || last_digit.is_none() {
            println!("No number found in {}", i);
            continue;
        }

        let combined = first_digit.map(|n| n*10 + last_digit.unwrap_or(0));
        sum += combined.unwrap_or(0);
    }
    println!("Sum: {}", sum);
}


fn get_first_digit(s: &str) -> Option<u32> {
    for (_, c) in s.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10);
        }
    }
    None
}

fn get_last_digit(s: &str) -> Option<u32> {
    for (_, c) in s.chars().rev().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10);
        }
    }
    None
}

fn part_two(input_file: &str) {
    let mut sum = 0;
    for i in read_to_string(input_file).unwrap().lines() {
        let first_digit = get_first_digit_two(i);
        let last_digit = get_last_digit_two(i);
        let combined = first_digit.map(|n| n*10 + last_digit.unwrap_or(0));
        sum += combined.unwrap_or(0);
    }
    println!("Sum_2: {}", sum);
}

fn get_text_number(s: &str, curr_index: usize) -> Option<u32> {
    let is_number : Vec<&str> = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    for (i, c) in is_number.iter().enumerate(){
        let n_len = c.len();
        let f = s.get(curr_index..curr_index+n_len);
        let th = f.map(|x| *c == x);
        if th.unwrap_or(false){
            return Some(i as u32);
        }
    }

    None
}

fn get_text_number_rev(s: &str, curr_index: usize) -> Option<u32> {
    let is_number : Vec<&str> = vec![
        "orez",
        "eno",
        "owt",
        "eerht",
        "ruof",
        "evif",
        "xis",
        "neves",
        "thgie",
        "enin",
    ];
    for (i, c) in is_number.iter().enumerate(){
        let n_len = c.len();
        let f = s.get(curr_index..curr_index+n_len);
        let th = f.map(|x| *c == x);
        if th.unwrap_or(false){
            return Some(i as u32);
        }
    }

    None
}

fn get_first_digit_two(s: &str) -> Option<u32> {
    for (i, c) in s.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10);
        }
        let maybe_digit = get_text_number(s, i);
        if maybe_digit.is_some() {
            return maybe_digit;
        }
    }
    None
}

fn get_last_digit_two(s: &str) -> Option<u32> {
    let rev = s.chars().rev().collect::<String>();
    for (i, c) in rev.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10);
        }
        let maybe_digit = get_text_number_rev(&rev, i);
        if maybe_digit.is_some() {
            return maybe_digit;
        }
    }
    None
}
