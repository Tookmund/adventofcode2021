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
struct Board {
    board: Vec<Vec<Num>>,
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
            .collect::<Vec<_>>()
        );
        let row = self.board.len()-1;
        for (col, val) in self.board.last().unwrap().iter().enumerate() {
            self.lookup.insert(*val, (row, col));
        }
    }
    fn mark(&self, n: Num) -> bool {
        true
    }
    fn check(&self, r: Num, c: Num) {
        ()
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
    let mut draws: Vec<Num>;
    let mut boards = Vec::<Board>::new();

    for line in io::stdin().lock().lines() {
        let line = line?;
        curstate = change_state(curstate, &line);
        match curstate {
            State::Draw => {
                draws = get_draws(&line);
                println!("{:?}", draws);
            },
            State::NewBoard => boards.push(Board::new()),
            State::PopulateBoard => {
                boards.last_mut().unwrap().populate(&line);
            }
        }
    }
    println!("{:?}", boards);
    Ok(())
}
