use std::fs::read_to_string;
struct LottoCard {
    numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
}

impl LottoCard {
    fn score_card(&self) -> u32 {
        let mut score = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }
    fn count_winning_numbers(&self) -> u32 {
        let mut count = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }
        count
    }
}

fn parse_lotto_numbers(line: &str) -> Vec<u8> {
    let mut numbers: Vec<u8> = vec![];
    for number in line.split_whitespace(){
        numbers.push(number.parse().unwrap())
    }
    numbers
}

fn parse_lotto_card(card: &str) -> LottoCard {
    let (winning_numbers_line, numbers_line) = get_lotto_lines(card);

    let winning_numbers = parse_lotto_numbers(winning_numbers_line);
    let numbers = parse_lotto_numbers(numbers_line);
    LottoCard { numbers, winning_numbers }
}

fn trim_lotto_line(line: &str) -> &str {
    line.split(':').collect::<Vec<&str>>()[1].trim()
}

fn get_lotto_lines(line: &str) ->  (&str, &str) {
    let lines = line.split('|').collect::<Vec<&str>>();
    (lines[0], lines[1])
}

fn part_1(lines: Vec<&str>) -> u32 {
    let mut score = 0;
    for line in lines {
        let card_line = trim_lotto_line(line);
        let card = parse_lotto_card(card_line);
        let card_score = card.score_card();
        score += card_score;
    }
    score
}

fn part_2(lines: Vec<&str>) -> u32 {
    let mut score = 0;
    let mut copies_won: Vec<u32> = vec![0; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let card_line = trim_lotto_line(line);
        let card = parse_lotto_card(card_line);
        let winning_numbers = card.count_winning_numbers();

        let copies_of_current_card: u32 = copies_won[i] + 1;
        for j in 1..(winning_numbers+1) {
            copies_won[i+j as usize] += 1 * copies_of_current_card;
        }
        score += copies_of_current_card;
    }
    score
}

fn main() {
    println!("Hello, world!");
    let lines = read_to_string("input.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    println!("Part 1: {}", part_1(str_lines.clone()));
    println!("Part 2: {}", part_2(str_lines.clone()));
}

#[test]
fn test_sample() {
    let lines = read_to_string("sample.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    assert_eq!(part_1(str_lines), 13);
}


#[test]
fn test_sample_part_2() {
    let lines = read_to_string("sample.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    assert_eq!(part_2(str_lines), 30);
}
