use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    env,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn solve(list : &Vec<i32>) -> Option<(i32, i32)> {
    for (i, a) in list.iter().enumerate() {
        for (j, b) in list.iter().enumerate() {
            if i == j { continue } ;
            if a + b == 2020 { return Some((*a, *b)) };
        }
    }
    return None
}


fn solve2(list : &Vec<i32>) -> Option<(i32, i32, i32)> {
    for (i, a) in list.iter().enumerate() {
        for (j, b) in list.iter().enumerate() {
            for (k, c) in list.iter().enumerate() {
                if i == j || i == k || j == k { continue } ;
                if a + b + c == 2020 { return Some((*a, *b, *c)) };
            }
        }
    }
    return None
}

fn main() {
    // get filename from args
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];

    // for each line of file
    let lines = lines_from_file(filename)
        .expect("Could not load lines from file");

    let v = lines.iter()
        .map(|s| s.parse::<i32>()
            .expect("could not parse to integer")).collect();

    let (a, b) = solve(&v)
        .expect("Could not find two pairs which sum to 2020");
    let (a2, b2, c2) = solve2(&v)
        .expect("Could not find three pairs which sum to 2020");

    println!("Part 1:");
    println!("a: {}, b: {}, answer: {}", a, b, a * b);
    println!("Part 2:");
    println!("a: {}, b: {}, c: {}, answer: {}", a2, b2, c2, a2 * b2 * c2);
}
