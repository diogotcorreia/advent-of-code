use std::{
    collections::VecDeque,
    io::{self, Read},
};

enum MarkResult {
    NotInBoard,
    Marked,
    Winner,
}

#[derive(Debug)]
struct BoardCell {
    num: u32,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    cells: Vec<BoardCell>,
    col_n: u8,
    row_n: u8,
    finished: bool,
}

impl Board {
    fn from(input: &str) -> Board {
        Board {
            cells: input
                .split_whitespace()
                .map(|v| BoardCell {
                    num: String::from(v).trim().parse().expect("board not a number"),
                    marked: false,
                })
                .collect(),
            col_n: 5,
            row_n: 5,
            finished: false,
        }
    }

    fn mark_number(&mut self, num: u32) -> MarkResult {
        let cell = match self
            .cells
            .iter_mut()
            .enumerate()
            .find(|(_, cell)| cell.num == num)
        {
            Some(v) => v,
            None => return MarkResult::NotInBoard,
        };

        cell.1.marked = true;

        let i: u8 = u8::try_from(cell.0).unwrap();

        if self.check_winner_horiz(i / self.row_n) || self.check_winner_vert(i % self.col_n) {
            return MarkResult::Winner;
        }
        MarkResult::Marked
    }

    fn check_winner_horiz(&self, row: u8) -> bool {
        for x in 0..self.col_n {
            let i: usize = (row * self.col_n + x).into();

            let cell: &BoardCell = self.cells.get(i).unwrap();
            if !cell.marked {
                return false;
            }
        }
        true
    }

    fn check_winner_vert(&self, col: u8) -> bool {
        for x in 0..self.row_n {
            let i: usize = (x * self.col_n + col).into();

            let cell: &BoardCell = self.cells.get(i).unwrap();
            if !cell.marked {
                return false;
            }
        }
        true
    }

    fn sum_unmarked_numbers(&self) -> u32 {
        self.cells
            .iter()
            .filter(|cell| !cell.marked)
            .map(|cell| cell.num)
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let mut draw_queue: VecDeque<u32> = input
        .split(',')
        .map(|v| v.trim().parse().expect("draw not a number"))
        .collect();

    input.clear();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read from stdin");

    let mut boards: Vec<Board> = input
        .split("\n\n")
        .map(|v| v.trim())
        .filter(|v| v.len() > 0)
        .map(|v| Board::from(v))
        .collect();

    let board_count = boards.len();
    let mut finished_boards: u8 = 0;

    loop {
        let drawn_number = match draw_queue.pop_front() {
            Some(v) => v,
            _ => break,
        };

        for board in boards.iter_mut() {
            if board.finished {
                continue;
            }
            match board.mark_number(drawn_number) {
                MarkResult::Winner => {
                    if finished_boards == 0 {
                        println!("Part 1: {}", board.sum_unmarked_numbers() * drawn_number);
                    } else if usize::from(finished_boards + 1) == board_count {
                        println!("Part 2: {}", board.sum_unmarked_numbers() * drawn_number);
                    }
                    board.finished = true;
                    finished_boards += 1;
                }
                _ => {}
            }
        }
    }
}
