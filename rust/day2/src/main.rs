use std::fs::read_to_string;
use verex::{Expression, Verex};

struct RecoveredPassword {
    policy: PasswordPolicy,
    password: String,
}

impl RecoveredPassword {
    const RECOVERED_PASSWORD_REGEX: Expression<'static> =
        Expression::String("(\\d*)-(\\d*) (\\p{L}): (.*)");

    pub fn from_string(rec_pass: String) -> Self {
        let mut engine: Verex = Verex::new();
        let compiled_regex = engine
            .capture_expr(RecoveredPassword::RECOVERED_PASSWORD_REGEX)
            .compile();
        let captures = compiled_regex
            .unwrap()
            .captures(&*rec_pass)
            .unwrap_or_else(|| panic!("failed parsing line: {}", rec_pass));

        let policy = PasswordPolicy {
            min: captures.at(2).unwrap().parse().unwrap(),
            max: captures.at(3).unwrap().parse().unwrap(),
            char: captures.at(4).unwrap().parse().unwrap(),
        };
        let recovered = RecoveredPassword {
            policy,
            password: captures.at(5).unwrap().to_string(),
        };
        recovered
    }

    fn validate_part1(&self) -> bool {
        self.policy.validate_password_part1(&self.password)
    }

    fn validate_part2(&self) -> bool {
        self.policy.validate_password_part2(&self.password)
    }
}

struct PasswordPolicy {
    min: i32,
    max: i32,
    char: char,
}

impl PasswordPolicy {
    fn validate_password_part1(&self, password: &str) -> bool {
        let char_count = password.matches(self.char).count() as i32;
        if char_count == 0 {
            return false;
        }
        char_count >= self.min && char_count <= self.max
    }

    fn validate_password_part2(&self, password: &str) -> bool {
        let matches: Vec<bool> = password.chars().map(|c| c == self.char).collect();

        matches[(self.min - 1) as usize] ^ matches[(self.max - 1) as usize]
    }
}

fn main() {
    let file = read_to_string("input.txt").expect("couldn't read input file");
    let lines = &file.lines().collect::<Vec<_>>();

    let start = std::time::Instant::now();
    let part1_valid_count = lines
        .iter()
        .filter(|l| RecoveredPassword::from_string(l.to_string()).validate_part1())
        .count();
    eprintln!("elapsed: {:?}", start.elapsed());
    println!("Part 1: found {} valid passwords", part1_valid_count);

    let start = std::time::Instant::now();
    let part2_valid_count = lines
        .iter()
        .filter(|l| RecoveredPassword::from_string(l.to_string()).validate_part2())
        .count();
    eprintln!("elapsed: {:?}", start.elapsed());
    println!("Part 2: found {} valid passwords", part2_valid_count);
}

#[test]
fn test_check_password_part1() {
    assert!(RecoveredPassword {
        policy: PasswordPolicy {
            min: 1,
            max: 3,
            char: 'a',
        },
        password: String::from("abcde"),
    }
    .validate_part1());

    assert!(!RecoveredPassword {
        policy: PasswordPolicy {
            min: 1,
            max: 3,
            char: 'b',
        },
        password: String::from("cdefg"),
    }
    .validate_part1());

    assert!(RecoveredPassword {
        policy: PasswordPolicy {
            min: 2,
            max: 9,
            char: 'c',
        },
        password: String::from("ccccccccc"),
    }
    .validate_part1());
}

#[test]
fn test_check_password_part2() {
    assert!(RecoveredPassword {
        policy: PasswordPolicy {
            min: 1,
            max: 3,
            char: 'a',
        },
        password: String::from("abcde"),
    }
    .validate_part2());

    assert!(!RecoveredPassword {
        policy: PasswordPolicy {
            min: 1,
            max: 3,
            char: 'b',
        },
        password: String::from("cdefg"),
    }
    .validate_part2());

    assert!(!RecoveredPassword {
        policy: PasswordPolicy {
            min: 2,
            max: 9,
            char: 'c',
        },
        password: String::from("ccccccccc"),
    }
    .validate_part2());
}

#[test]
fn test_recovered_password_parser() {
    let rec_password = String::from("1-3 a: abcde");
    let parsed_rec_password = RecoveredPassword::from_string(rec_password);
    assert_eq!(parsed_rec_password.password, "abcde");
    assert_eq!(parsed_rec_password.policy.char, 'a');
    assert_eq!(parsed_rec_password.policy.min, 1);
    assert_eq!(parsed_rec_password.policy.max, 3);
}
