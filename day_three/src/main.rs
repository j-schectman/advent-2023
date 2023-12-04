use std::fs::read_to_string;
fn main() {
    let lines = read_to_string("jd_sample.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    println!("part 1: {}", part_1(str_lines));
}

struct Point(i32, i32);

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
    let mut symbols: Vec<Point> = vec![];
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
                println!("x{} y{} {}", j, i, c);
                symbols.push(Point(j as i32, i as i32));
            }
        }
        if part.is_some() {
            part_points[i].push(part.unwrap());
        }
    }

    let mut sum = 0;
    for symbol in symbols.iter(){
        let Point(x, y) = *symbol;
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

            println!("testing x{} y{}, columns{}", x, y, columns);
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
