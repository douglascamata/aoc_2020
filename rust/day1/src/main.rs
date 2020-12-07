use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    // I got curious about comparing the performances of borrowing and not-borrowing so I
    // implemented both functions and a very rudimentar timer.
    let file = read_to_string("day1/input.txt").expect("couldn't read input file");
    let lines = file.lines().collect::<Vec<_>>();
    println!("Input size (in lines): {}", file.lines().count());
    println!("No borrow:");
    println!("Part 1: {}", day1_no_borrow(lines.clone(), 2));
    println!("Part 2: {}", day1_no_borrow(lines.clone(), 3));
    println!("{}", "-".repeat(80));
    println!("Borrow:");
    println!("Part 1: {}", day1_borrow(&lines, 2));
    println!("Part 2: {}", day1_borrow(&lines, 3));
}

fn day1_no_borrow(lines: Vec<&str>, comb: usize) -> i32 {
    let start = std::time::Instant::now();
    let numbers = lines.iter().map(|n| n.parse::<i32>().unwrap());
    let combinations = numbers.combinations(comb);

    let result = combinations
        .into_iter()
        .find(|numbers| numbers.iter().sum::<i32>() == 2020)
        .map(|numbers| numbers.into_iter().product::<i32>())
        .unwrap();
    eprintln!("elapsed: {:?}", start.elapsed());
    result as i32
}

fn day1_borrow(lines: &[&str], comb: usize) -> i32 {
    let start = std::time::Instant::now();
    let numbers = lines.iter().map(|n| n.parse::<i32>().unwrap());
    let combinations = numbers.combinations(comb);

    let result = combinations
        .into_iter()
        .find(|numbers| numbers.iter().sum::<i32>() == 2020)
        .map(|numbers| numbers.into_iter().product::<i32>())
        .unwrap();
    eprintln!("elapsed: {:?}", start.elapsed());
    result as i32
}

#[test]
fn test_day1() {
    let base_input = vec!["1721", "979", "366", "299", "675", "1456"];
    assert_eq!(day1_no_borrow(base_input.clone(), 2), 514579);
    assert_eq!(day1_no_borrow(base_input.clone(), 3), 241861950);

    assert_eq!(day1_borrow(&base_input, 2), 514579);
    assert_eq!(day1_borrow(&base_input, 3), 241861950);
}
