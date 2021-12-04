use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum State {
    Draw,
    NewBoard,
    PopulateBoard
}

type Num = usize;

#[derive(Debug)]
struct Cell {
    num: Num,
    mark: bool,
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<Cell>>,
    lookup: HashMap<Num, (usize, usize)>,
}

impl Board {
    fn new() -> Self {
        Board {
            board: Vec::new(),
            lookup: HashMap::new(),
        }
    }
    fn populate(&mut self, line: &str) {
        self.board.push(
            line.split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .map(|n| Cell { num: n, mark: false })
            .collect::<Vec<_>>()
        );
        let row = self.board.len()-1;
        for (col, val) in self.board.last().unwrap().iter().enumerate() {
            self.lookup.insert(val.num, (row, col));
        }
    }
    // Returns true if marking that number causes this board to win
    fn mark(&mut self, n: Num) -> bool {
        match self.lookup.get(&n) {
            None => false,
            Some((r, c)) if self.board[*r][*c].mark == true => panic!("Marking cell ({}, {}) twice!", r, c),
            Some((r, c)) => {
                self.board[*r][*c].mark = true;
                self.check(*r, *c)
            }
        }
    }
    fn check(&self, r: Num, c: Num) -> bool {
        if self.board[r][c].mark != true {
            panic!("Checking unmarked cell ({}, {})!", r, c);
        }
        self.check_row(r) || self.check_col(c)
    }
    fn check_col(&self, c: Num) -> bool {
        let mut win = true;
        for ir in 0..self.board.len() {
            if !self.board[ir][c].mark {
                win = false;
            }
        }
        win
    }
    fn check_row(&self, r: Num) -> bool {
        let mut win = true;
        for ic in 0..self.board[0].len() {
            if !self.board[r][ic].mark {
                win = false;
            }
        }
        win
    }
    fn sum_unmarked(&self) -> Num {
        self.board.iter().flatten()
            .filter(|n| n.mark == false)
            .map(|n| n.num)
            .sum()
    }
}

fn change_state(curstate: State, line: &str) -> State {
    match curstate {
        State::Draw if line == "" => State::NewBoard,
        State::NewBoard => State::PopulateBoard,
        State::PopulateBoard if line == "" => State::NewBoard,
        _ => curstate,
    }
}

fn get_draws(line: &str) -> Vec<Num> {
    line.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn main() -> io::Result<()>{
    let mut curstate = State::Draw;
    let mut draws: Vec<Num> = Vec::new();
    let mut boards = Vec::<Board>::new();

    for line in io::stdin().lock().lines() {
        let line = line?;
        curstate = change_state(curstate, &line);
        match curstate {
            State::Draw => draws = get_draws(&line),
            State::NewBoard => boards.push(Board::new()),
            State::PopulateBoard => {
                boards.last_mut().unwrap().populate(&line);
            }
        }
    }

    for d in draws {
        for b in boards.iter_mut() {
            if b.mark(d) {
                println!("Final Score: {}", b.sum_unmarked()*d);
                return Ok(());
            }
        }
    }
    Ok(())
}
