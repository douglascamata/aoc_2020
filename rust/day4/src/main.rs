use std::fs::read_to_string;
use std::time;
use verex::{Expression as E, Verex};

fn main() {
    let input_string = read_to_string("./day4/input.txt").expect("couldn't read input file");
    let start = time::Instant::now();
    let results = CredentialsValidator::validate_batch(&input_string);
    let total = results.len();
    let valid_passports = results.iter().filter(|&&r| r).count();
    eprintln!("elapsed: {:?}", start.elapsed());
    println!("found {} valid passports out of {}", valid_passports, total);
}

type CredentialInput = String;

struct Credentials {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Credentials {
    const FIELD_VALIDATION_FUNCS: [fn(&Credentials) -> bool; 7] = [
        Self::valid_byr,
        Self::valid_hgt,
        Self::valid_ecl,
        Self::valid_eyr,
        Self::valid_hcl,
        Self::valid_iyr,
        Self::valid_pid,
    ];

    fn has_valid_fields(&self) -> bool {
        Self::FIELD_VALIDATION_FUNCS.iter().all(|f| f(self))
    }

    fn valid_byr(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
    }

    fn valid_iyr(&self) -> bool {
        self.iyr >= 2010 && self.iyr <= 2020
    }

    fn valid_eyr(&self) -> bool {
        self.eyr >= 2020 && self.eyr <= 2030
    }

    fn valid_hgt(&self) -> bool {
        let height_regex = Verex::new()
            .capture_expr(E::String("^\\d+"))
            .compile()
            .unwrap();
        let height = match height_regex.captures(self.hgt.as_str()) {
            None => 0,
            Some(captures) => captures.at(0).unwrap().parse().unwrap(),
        };

        let unit_regex = Verex::new()
            .capture_expr(E::String("cm|in"))
            .compile()
            .unwrap();
        let unit = match unit_regex.captures(self.hgt.as_str()) {
            None => "",
            Some(captures) => captures.at(0).unwrap(),
        };

        match unit {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false,
        }
    }

    fn valid_hcl(&self) -> bool {
        let hcl_regex = Verex::new()
            .capture_expr(E::String("^#[a-f|0-9]{6}$"))
            .compile()
            .unwrap();
        hcl_regex.captures(self.hcl.as_str()).is_some()
    }

    fn valid_ecl(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&*self.ecl)
    }

    fn valid_pid(&self) -> bool {
        let pid_regex = Verex::new()
            .capture_expr(E::String("^\\d{9}$"))
            .compile()
            .unwrap();
        pid_regex.captures(self.pid.as_str()).is_some()
    }
}

impl From<CredentialInput> for Credentials {
    fn from(input: CredentialInput) -> Self {
        let mut credentials = Self {
            byr: 0,
            iyr: 0,
            eyr: 0,
            hgt: "".to_string(),
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: "".to_string(),
        };
        input
            .split(' ')
            .map(|key_value| key_value.split(':').collect::<Vec<&str>>())
            .for_each(|values| match values[0] {
                "byr" => credentials.byr = values[1].parse().unwrap(),
                "iyr" => credentials.iyr = values[1].parse().unwrap(),
                "eyr" => credentials.eyr = values[1].parse().unwrap(),
                "hgt" => credentials.hgt = values[1].parse().unwrap(),
                "hcl" => credentials.hcl = values[1].parse().unwrap(),
                "ecl" => credentials.ecl = values[1].parse().unwrap(),
                "pid" => credentials.pid = values[1].parse().unwrap(),
                _ => {}
            });
        credentials
    }
}

struct CredentialsValidator {
    input: CredentialInput,
}

impl CredentialsValidator {
    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn is_valid(&self) -> bool {
        self.is_valid_credential() && Credentials::from(self.input.clone()).has_valid_fields()
    }

    fn is_valid_credential(&self) -> bool {
        Self::REQUIRED_FIELDS
            .iter()
            .all(|field| self.input.contains(field))
    }

    fn validate_batch(input: &str) -> Vec<bool> {
        let mut validation_results: Vec<bool> = vec![];
        let mut reader_acc = String::new();

        for line in input.lines().map(|line| line.trim()) {
            if line.is_empty() && !reader_acc.is_empty() {
                let validation_result = Self { input: reader_acc }.is_valid();
                validation_results.push(validation_result);
                reader_acc = String::new();
                continue;
            }
            reader_acc.push(' ');
            reader_acc.push_str(line);
        }
        validation_results.push(Self { input: reader_acc }.is_valid());

        validation_results
    }
}

#[test]
fn test_credentials_validation() {
    let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    let expected_result = [true, false, true, false];

    assert_eq!(CredentialsValidator::validate_batch(input), expected_result);
}

#[test]
fn test_credential_parser() {
    let input: CredentialInput =
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string();
    let creds = Credentials::from(input);

    assert_eq!(creds.iyr, 2013)
}

#[test]
fn test_field_validation() {
    let mut cred = Credentials {
        byr: 0,
        iyr: 0,
        eyr: 0,
        hgt: "".to_string(),
        hcl: "".to_string(),
        ecl: "".to_string(),
        pid: "".to_string(),
    };

    assert!(!cred.valid_byr());
    cred.byr = 2002;
    assert!(cred.valid_byr());

    assert!(!cred.valid_hgt());
    cred.hgt = String::from("190cm");
    assert!(cred.valid_hgt());
    cred.hgt = String::from("60in");
    assert!(cred.valid_hgt());
}

#[test]
fn test_full_passport_validation() {
    let invalid_input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

    let results = CredentialsValidator::validate_batch(invalid_input);
    assert!(results.iter().all(|&r| !r));

    let valid_input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    let results = CredentialsValidator::validate_batch(valid_input);
    assert!(results.iter().all(|&r| r))
}
