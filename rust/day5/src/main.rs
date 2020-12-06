use std::fs::read_to_string;

fn main() {
    let input_string = read_to_string("day5/input.txt").expect("couldn't read input file");
    let start = std::time::Instant::now();
    let seat_ids: &Vec<i32> = &input_string
        .lines()
        .map(|l| SeatCode::from(l.trim().to_string()).seat_id())
        .collect();

    println!("max seat id: {}", seat_ids.into_iter().max().unwrap());

    for seat_id in seat_ids {
        let next_id_empty = seat_ids.into_iter().all(|id| *id != seat_id + 1);
        let next_next_busy = seat_ids.into_iter().any(|id| *id == seat_id + 2);
        if next_id_empty && next_next_busy {
            eprintln!("Your seat id is: {}", seat_id + 1);
            break;
        }
    }
    eprintln!("elapsed: {:?}", start.elapsed());
}

struct SeatCode {
    row_code: Vec<char>,
    column_code: Vec<char>,
}

impl SeatCode {
    const MAX_ROW: i32 = 127;
    const MAX_COLUMN: i32 = 7;

    fn find_row(&self) -> i32 {
        self.binary_space_partition(Self::MAX_ROW.clone(), 'F', 'B', &self.row_code)
    }

    fn find_column(&self) -> i32 {
        self.binary_space_partition(Self::MAX_COLUMN.clone(), 'L', 'R', &self.column_code)
    }

    fn seat_id(&self) -> i32 {
        (self.find_row() * 8) + self.find_column()
    }

    fn binary_space_partition(
        &self,
        max: i32,
        low_half_code: char,
        upper_half_code: char,
        input: &Vec<char>,
    ) -> i32 {
        let mut upper_bound = max;
        let mut lower_bound = 0;
        let mut last_read = ' ';
        // println!("{}", "-".repeat(80));
        // println!("solving for {:?}", self.row_code);
        // println!("bounds: {}-{}", lower_bound, upper_bound);
        for c in input {
            last_read = *c;
            if last_read == low_half_code {
                upper_bound -= (((upper_bound - lower_bound) as f64) / 2.0).ceil() as i32
            } else if last_read == upper_half_code {
                lower_bound += (((upper_bound - lower_bound) as f64) / 2.0).ceil() as i32
            }
            // println!("bounds: {}-{}", lower_bound, upper_bound);
        }
        if last_read == low_half_code {
            lower_bound
        } else if last_read == upper_half_code {
            upper_bound
        } else {
            0
        }
    }
}

impl From<String> for SeatCode {
    fn from(input: String) -> Self {
        Self {
            row_code: input[0..7].chars().collect(),
            column_code: input[7..input.len()].chars().collect(),
        }
    }
}

#[test]
fn test_basic_seat_code() {
    let code = String::from("BFFFBBFRRR");
    let seat = SeatCode::from(code);

    assert_eq!(seat.row_code, "BFFFBBF".chars().collect::<Vec<char>>());
    assert_eq!(seat.column_code, "RRR".chars().collect::<Vec<char>>());

    assert_eq!(seat.find_row(), 70);
    assert_eq!(seat.find_column(), 7);
    assert_eq!(seat.seat_id(), 567);
}

#[test]
fn test_seat_find_row() {
    let seat = SeatCode::from("FFFBBBFRRR".to_string());
    assert_eq!(seat.find_row(), 14);
    assert_eq!(seat.find_column(), 7);
    assert_eq!(seat.seat_id(), 119);

    let seat = SeatCode::from("BBFFBBFRLL".to_string());
    assert_eq!(seat.find_row(), 102);
    assert_eq!(seat.find_column(), 4);
    assert_eq!(seat.seat_id(), 820);
}
