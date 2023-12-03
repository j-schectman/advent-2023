use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
struct Draw {
    color: String,
    count: u32,
}

fn main() {
    let path = "input_one.txt";
    part_one(path);
    part_two(path);
}

fn part_one(path: &str) {
    let test_dict = HashMap::from([
                                  ("red", 12),
                                  ("green", 13),
                                  ("blue", 14),
    ]);
    let mut count = 0;
    for (i, line) in read_to_string(path).unwrap().lines().enumerate() {
        let trimmed = trim(line);
        let rounds = get_round_strings(trimmed);
        let mut valid = true;
        for round in rounds {
            let draws = get_draw_strings(round);
            for draw in draws {
                let s_draw = make_draw(draw);
                let in_bag = test_dict.get(&s_draw.color.as_str());
                if in_bag.is_none() || in_bag.unwrap() < &s_draw.count{
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            count += i + 1;
        }
    }

    println!("Part One: {}", count);
}

fn part_two(path: &str) {
    let mut count = 0;

    for line in read_to_string(path).unwrap().lines() {
        let mut test_dict = HashMap::from([
                                      ("red", 0 as u32),
                                      ("green", 0 as u32),
                                      ("blue", 0 as u32),
        ]);
        let trimmed = trim(line);
        let rounds = get_round_strings(trimmed);
        for round in rounds {
            let draws = get_draw_strings(round);
            for draw in draws {
                let s_draw = make_draw(draw);
                let color = s_draw.color;
                let in_bag = test_dict.get_mut(color.as_str());
                if in_bag.is_none() {
                    continue;
                }
                let bag_count = in_bag.unwrap();
                if *bag_count < s_draw.count{
                    *bag_count = s_draw.count;
                }
            }
        }

        let mut power = 1;
        for (_, value) in test_dict {
            if value == 0 {
                continue;
            }
            power *= value;
        }
        count += power;
    }

    println!("Part Two: {}", count);
}

fn trim(s: &str) -> &str {
    let mut rounds = s.split(":");
    // "game"
    rounds.next();
    let ret = rounds.next();
    if ret.is_none() {
        return "";
    }
    return ret.unwrap()
}

fn get_round_strings(s: &str) -> Vec<&str> {
    s.split(";").collect::<Vec<&str>>()
}

fn get_draw_strings(s: &str) -> Vec<&str> {
    s.split(",").collect::<Vec<&str>>()
}

fn make_draw(s: &str) -> Draw {
    let mut vals = s.split_whitespace();
    let count = vals.next().and_then(|x| x.parse::<u32>().ok());
    let color = vals.next().unwrap();
    Draw {
        color: color.to_string(),
        count: count.unwrap_or(0),
    }


}
