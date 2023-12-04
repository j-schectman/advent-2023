use std::fs::read_to_string;

// YOU SHOULD REFACTOR
fn main() {
    let lines = read_to_string("input.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    println!("part 1: {}", part_1(str_lines));
}

#[derive(Clone)]
struct Symbol {
    x: i32,
    y: i32,
    char_symbol: char,
}

#[derive(Clone)]
struct Part{
    x_start: i32,
    x_end: i32,
    id: i32,
    counted: bool,
}

fn part_1(lines: Vec<&str>) -> i32 {
    let rows = lines.len();
    let columns = lines[0].len();
    let mut symbols: Vec<Symbol> = vec![];
    let mut part_points: Vec<Vec<Part>> = vec![Vec::with_capacity(columns); rows];
    for (i, line) in lines.iter().enumerate() {
        let mut part = None::<Part>;
        for (j, c) in line.chars().enumerate() {
            if c.is_numeric() {
                part = part.map_or(Some(Part{
                    x_start: j as i32,
                    x_end: j as i32,
                    id: c.to_digit(10).unwrap() as i32,
                    counted: false,
                }), |mut p| {
                    p.x_end = j as i32;
                    p.id = p.id * 10 + c.to_digit(10).unwrap() as i32;
                    Some(p)
                });
            } else if part.is_some() {
                part_points[i].push(part.unwrap());
                part = None;
            }
            if c != '.' && !c.is_digit(10) {
                symbols.push(Symbol{
                    x: j as i32,
                    y: i as i32,
                    char_symbol: c
                });
            }
        }
        if part.is_some() {
            part_points[i].push(part.unwrap());
        }
    }
    
    part_2_calc(part_points.clone(), symbols.clone());
    part_1_calc(part_points.clone(), symbols.clone(), rows)
}

fn part_1_calc(mut part_points: Vec<Vec<Part>>, symbols: Vec<Symbol>, rows: usize) -> i32 {
    let mut sum = 0;
    for o_symbol in symbols.iter(){
        let Symbol{ x, y, char_symbol: _ } = *o_symbol;
        if y > 0 {
            let parts_above = &mut part_points[(y as usize)-1];
            for part in parts_above.iter_mut() {
                if x >= part.x_start - 1 && x <= part.x_end + 1 && !part.counted {
                    sum += part.id;
                    part.counted = true;
                }
            }
        }

        if y < (rows - 1) as i32{
            let parts_below = &mut part_points[(y as usize)+1];
            for part in parts_below.iter_mut(){
                if x >= part.x_start - 1 && x <= part.x_end + 1 && !part.counted {
                    sum += part.id;
                    part.counted = true;
                }
            }
        }

        if y < rows as i32 {
            let parts_inline = &mut part_points[y as usize];
            for part in parts_inline.iter_mut() {
                if x >= part.x_start-1 && x <= part.x_end+1 && !part.counted{
                    sum += part.id;
                    part.counted = true;

                }
            }
        }
    }
    sum
}

fn part_2_calc (mut part_points: Vec<Vec<Part>>, symbols: Vec<Symbol>) -> i32 {
    let mut sum = 0;
    for o_symbol in symbols.iter(){
        let Symbol{ x, y, char_symbol } = *o_symbol;
        let mut found_parts: Vec<Part> = vec![];
        if y > 0 {
            let parts_above = &mut part_points[(y as usize)-1];
            for part in parts_above.iter_mut() {
                if x >= part.x_start - 1 && x <= part.x_end + 1 && char_symbol == '*'{
                    found_parts.push(part.clone());
                }
            }
        }

        if y < (part_points.len() - 1) as i32{
            let parts_below = &mut part_points[(y as usize)+1];
            for part in parts_below.iter_mut(){
                if x >= part.x_start - 1 && x <= part.x_end + 1 && char_symbol == '*'{
                    found_parts.push(part.clone());
                }
            }
        }

        if y < part_points.len() as i32 {
            let parts_inline = &mut part_points[y as usize];
            for part in parts_inline.iter_mut() {
                if x >= part.x_start-1 && x <= part.x_end+1 && char_symbol == '*'{
                    found_parts.push(part.clone());
                }
            }
        }
        if found_parts.len() == 2 {
            match &found_parts[..] {
                [part1, part2] => {
                    sum += part1.id * part2.id;
                },
                _ => panic!("found more than 2 parts"),
            }
        }
    }
    println!("part_2 gear ratios: {}", sum);
    sum
}


#[test]
fn test_sample() {
    let lines = read_to_string("sample.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    assert_eq!(part_1(str_lines), 4361);
}

#[test]
fn test_center() {
    let lines = read_to_string("test1.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    assert_eq!(part_1(str_lines), 6);
}

#[test]
fn test_another() {
    let lines = read_to_string("test2.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    assert_eq!(part_1(str_lines), 130);
}

