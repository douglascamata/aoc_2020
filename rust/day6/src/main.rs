use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let input = read_to_string("day6/input.txt").expect("couldn't read input file");
    let group_answers = &parse_input_lines(input.lines().collect());
    let start = Instant::now();
    let total_unique_questions_answered: i32 = group_answers
        .iter()
        .map(|g| g.unique_questions_answered())
        .sum();
    println!(
        "total questions answered: {}",
        total_unique_questions_answered
    );

    let count_questions_all_yes_per_group: i32 = group_answers
        .iter()
        .map(|g| g.all_yes_question_count())
        .sum();
    println!(
        "total amount of questions that got yes from whole group: {}",
        count_questions_all_yes_per_group,
    );
    eprintln!("elapsed: {:?}", start.elapsed());
}

struct GroupAnswers<'a> {
    answers: Vec<&'a str>,
}

impl GroupAnswers<'_> {
    fn unique_questions_answered(&self) -> i32 {
        let mut unique_questions_answered: HashMap<char, bool> = HashMap::new();
        self.answers.iter().for_each(|&member_answers| {
            member_answers.chars().for_each(|c| {
                if !unique_questions_answered.contains_key(&c) {
                    unique_questions_answered.insert(c, true);
                };
            })
        });
        unique_questions_answered.len() as i32
    }

    fn all_yes_question_count(&self) -> i32 {
        let mut unique_questions_answered: HashMap<char, i32> = HashMap::new();
        self.answers.iter().for_each(|&member_answers| {
            member_answers.chars().for_each(|c| {
                let counter = unique_questions_answered.entry(c).or_default();
                *counter += 1
            })
        });
        unique_questions_answered
            .values()
            .filter(|&&v| v == self.answers.len() as i32)
            .count() as i32
    }
}

fn parse_input_lines(input: Vec<&str>) -> Vec<GroupAnswers> {
    let mut line_acc: Vec<&str> = Vec::new();
    let mut answers: Vec<GroupAnswers> = Vec::new();
    input.into_iter().map(|line| line.trim()).for_each(|line| {
        if line.is_empty() {
            if !line_acc.is_empty() {
                answers.push(GroupAnswers {
                    answers: line_acc.clone(),
                });
                line_acc = vec![];
            }
        } else {
            line_acc.push(line);
        }
    });
    if !line_acc.is_empty() {
        answers.push(GroupAnswers {
            answers: line_acc.clone(),
        });
    }
    answers
}

#[test]
fn test_group_answers_count() {
    let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
    "#;
    let input_lines = input.lines().collect();
    let parsed_lines = parse_input_lines(input_lines);
    let total_unique_answers: i32 = parsed_lines
        .iter()
        .map(|g| g.unique_questions_answered())
        .sum();
    assert_eq!(total_unique_answers, 11);

    let part2: i32 = parsed_lines
        .iter()
        .map(|g| g.all_yes_question_count())
        .sum();
    assert_eq!(part2, 6)
}
