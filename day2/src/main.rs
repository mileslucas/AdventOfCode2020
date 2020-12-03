use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use std::env;
use regex::Regex;


fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn count(ch : char, input : &str) -> i32 {
    let mut n = 0;
    for x in input.chars() {
        if x.eq_ignore_ascii_case(&ch) { n += 1; };
    }
    return n
}

fn parse_line(line : &str) -> Option<(i32, i32, char, String)> {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (.+)$").unwrap();
    let caps = re.captures(line)
        .expect("Could not capture from input");

    if caps.len() != 5 { return None };

    let min = caps[1].parse::<i32>().unwrap();
    let max = caps[2].parse::<i32>().unwrap();
    let sent = caps[3].chars().next().unwrap();
    let target = caps[4].to_string();

    return Some((min, max, sent, target));
}

fn part1(lines: &Vec<String>) {

    let mut total_valid = 0;

    for line in lines {
        let (min, max, sent, target) = parse_line(&line)
            .expect("Could not parse line");
        let cnt = count(sent, &target);
        if min <= cnt && cnt <= max { total_valid += 1 };
    }

    println!("Part 1: {} valid inputs", total_valid);
}

fn part2(lines: &Vec<String>) {
    let mut total_valid = 0;
    for line in lines {
        let (i, j, sent, target) = parse_line(&line)
            .expect("Could not parse line");
        let c = target.as_bytes();
        let a = c[(i - 1) as usize];
        let b = c[(j - 1) as usize];
        if (a == sent as u8) ^ (b == sent as u8) {
            total_valid +=  1;
        }
    }

    println!("Part 2: {} valid inputs", total_valid);
}

fn main() {
    // get filename from args
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];

    // for each line of file
    let lines = lines_from_file(filename)
        .expect("Could not load lines from file");

    //// Test input
    // let lines = vec!["1-3 a: abcde",
    // "1-3 b: cdefg",
    // "2-9 c: ccccccccc"];
    part1(&lines);
    part2(&lines);
}