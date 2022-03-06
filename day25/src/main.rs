use std::io;
use std::io::prelude::*;
struct Location {
    row: usize,
    col: usize
}

struct Move {
    old: Location,
    new: Location
}

#[derive(Debug)]
enum Cucumber {
    Empty,
    East,
    South
}

#[derive(Debug)]
struct SeaFloor(Vec<Vec<Cucumber>>);

impl SeaFloor {
    fn new<B: io::BufRead>(bufread: B) -> io::Result<SeaFloor> {
        let mut seafloor = Vec::new();
        for line in bufread.lines() {
            let mut cur = Vec::new();
            for ch in line?.chars() {
                cur.push(match ch {
                    '.' => Cucumber::Empty,
                    '>' => Cucumber::East,
                    'v' => Cucumber::South,
                    invalid => panic!("Invalid Cucumber: {}", invalid)
                });
            }
            seafloor.push(cur);
        }
        Ok(SeaFloor(seafloor))
    }

    fn run(&mut self) -> usize {
        let mut i = 0;
        loop {
            i += 1;
            let mut east = Vec::new();
            for r in 0..self.0.len() {
                for c in 0..self.0[r].len() {
                    match self.0[r][c] {
                        Cucumber::East => {
                            let newc = (c+1) % self.0[r].len();
                            match self.0[r][newc] {
                                Cucumber::Empty => {
                                    east.push(Move {
                                        old: Location { row: r, col: c },
                                        new: Location { row: r, col: newc }
                                    })
                                },
                                _ => (),
                            }
                        },
                        _ => ()
                    }
                }
            }
            for m in &east {
                self.0[m.old.row][m.old.col] = Cucumber::Empty;
                self.0[m.new.row][m.new.col] = Cucumber::East;
            }
            let mut south = Vec::new();
            for r in 0..self.0.len() {
                for c in 0..self.0[r].len() {
                    match self.0[r][c] {
                        Cucumber::South => {
                            let newr = (r+1) % self.0.len();
                            match self.0[newr][c] {
                                Cucumber::Empty => {
                                    south.push(Move {
                                        old: Location { row: r, col: c },
                                        new: Location { row: newr, col: c }
                                    })
                                },
                                _ => (),
                            }
                        },
                        _ => ()
                    };
                }
            }
            for m in &south {
                self.0[m.old.row][m.old.col] = Cucumber::Empty;
                self.0[m.new.row][m.new.col] = Cucumber::South;
            }
            if east.is_empty() && south.is_empty() {
                break;
            }
        }
        i
    }
}

#[cfg(test)]
mod test {
    use crate::function;

    const EXAMPLE: &[u8] = b"\
TEST\n\
Data\n\
Here\n";

    #[test]
    fn test_example() {
        assert_eq!(function(EXAMPLE).unwrap(), 0)
    }
}

fn main() -> io::Result<()> {
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    let mut seafloor = SeaFloor::new(io::stdin().lock())?;
    println!("SeaFloor: {:?}", seafloor);
    println!("Run: {}", seafloor.run());
    Ok(())
}
