use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct RingBuffer<T> {
    array: Vec<T>,
}

impl RingBuffer<char> {
    fn from_string(input: &str) -> Self {
        let input_list = input.chars().collect::<Vec<char>>();
        RingBuffer { array: input_list }
    }
}

impl Index<usize> for RingBuffer<char> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index % self.array.len()]
    }
}

impl IndexMut<usize> for RingBuffer<char> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let max = self.array.len();
        &mut self.array[index % max]
    }
}

#[derive(Clone, Copy, Debug)]
enum HorizontalMove {
    Right(usize),
}

#[derive(Clone, Copy, Debug)]
enum VerticalMove {
    Down(usize),
}

#[derive(Debug)]
struct Toboggan {
    pos_x: usize,
    pos_y: usize,
    tree_count: usize,
    map: Vec<RingBuffer<char>>,
}

impl Toboggan {
    fn from_vec(input: &[&str]) -> Self {
        let map_lines = input
            .iter()
            .map(|line| line.trim())
            .filter(|&line| !line.is_empty())
            .map(|line| RingBuffer::from_string(line))
            .collect();
        Toboggan {
            pos_x: 0,
            pos_y: 0,
            tree_count: 0,
            map: map_lines,
        }
    }

    fn move_until_finished_to(&mut self, move_x: &HorizontalMove, move_y: &VerticalMove) {
        for _ in 1..self.map.len() {
            self.move_to(*move_x, *move_y);
        }
    }

    fn move_to(&mut self, move_x: HorizontalMove, move_y: VerticalMove) {
        let (dest_y, dest_x) = self.parse_moves(move_x, move_y);
        if dest_y >= self.map.len() {
            return;
        }
        self.do_move(dest_y, dest_x);
    }

    fn parse_moves(&self, move_x: HorizontalMove, move_y: VerticalMove) -> (usize, usize) {
        let dest_y = {
            match move_y {
                VerticalMove::Down(amount) => self.pos_y + amount,
            }
        };
        let dest_x = {
            match move_x {
                HorizontalMove::Right(amount) => self.pos_x + amount,
            }
        };
        (dest_y, dest_x)
    }

    fn do_move(&mut self, dest_y: usize, dest_x: usize) {
        let arrival_change = self.process_arrival(dest_y, dest_x);
        self.map[dest_y][dest_x] = arrival_change;
        self.pos_x = dest_x;
        self.pos_y = dest_y;
    }

    fn process_arrival(&mut self, dest_y: usize, dest_x: usize) -> char {
        let at_destination = self.map[dest_y][dest_x];
        match at_destination {
            '#' => {
                self.tree_count += 1;
                'X'
            }
            _ => 'O',
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    let input_vec: Vec<&str> = input.lines().collect();
    let mut total_tree_mult = 1;

    let moves: Vec<(HorizontalMove, VerticalMove)> = vec![
        (HorizontalMove::Right(1), VerticalMove::Down(1)),
        (HorizontalMove::Right(3), VerticalMove::Down(1)),
        (HorizontalMove::Right(5), VerticalMove::Down(1)),
        (HorizontalMove::Right(7), VerticalMove::Down(1)),
        (HorizontalMove::Right(1), VerticalMove::Down(2)),
    ];

    moves.into_iter().for_each(|(move_x, move_y)| {
        let mut toboggan = Toboggan::from_vec(&input_vec);
        toboggan.move_until_finished_to(&move_x, &move_y);
        println!("{}", toboggan.tree_count);
        if toboggan.tree_count != 0 {
            total_tree_mult *= toboggan.tree_count;
        }
    });

    println!("Total tree mult: {}", total_tree_mult);
}

#[test]
fn test_toboggan_single_slope() {
    let slope_test_input = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
    "#;

    let mut toboggan = Toboggan::from_vec(&slope_test_input.lines().collect::<Vec<&str>>());
    toboggan.move_until_finished_to(&HorizontalMove::Right(3), &VerticalMove::Down(1));
    assert_eq!(toboggan.tree_count, 7);
}

#[test]
fn test_toboggan_multi_slope() {
    let slope_test_input = r#"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
    "#;
    let input_vec: Vec<&str> = slope_test_input.lines().collect();
    let mut total_tree_count = 0;

    let moves: Vec<(HorizontalMove, VerticalMove, usize)> = vec![
        (HorizontalMove::Right(1), VerticalMove::Down(1), 2),
        (HorizontalMove::Right(3), VerticalMove::Down(1), 7),
        (HorizontalMove::Right(5), VerticalMove::Down(1), 3),
        (HorizontalMove::Right(7), VerticalMove::Down(1), 4),
        (HorizontalMove::Right(1), VerticalMove::Down(2), 2),
    ];

    moves
        .into_iter()
        .for_each(|(move_x, move_y, expected_tree_count)| {
            let mut toboggan = Toboggan::from_vec(&input_vec);
            toboggan.move_until_finished_to(&move_x, &move_y);
            total_tree_count += toboggan.tree_count;
            assert_eq!(
                toboggan.tree_count, expected_tree_count,
                "failed at move {:?} {:?}",
                move_x, move_y
            );
        });
}

#[test]
fn test_ring_buffer() {
    let ring = RingBuffer::from_string("foo");
    assert_eq!(ring[0], 'f');
    assert_eq!(ring[3], 'f');
}
