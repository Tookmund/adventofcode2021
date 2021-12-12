use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::fmt;

type Energy = u32;
type FlashSet = HashSet<(usize, usize)>;

#[derive(Debug)]
struct OctopusGrid {
    grid: Vec<Vec<Energy>>,
}

impl OctopusGrid {
    fn new<B: io::BufRead>(bufread: B) -> Self {
        OctopusGrid {
            grid: bufread.lines().map(|l| {
                l.unwrap().chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<Energy>>()
            }).collect(),
        }
    }

    fn step_num(&mut self, steps: usize) -> usize {
        println!("Before any steps:\n{}", self);
        let mut flashes = 0;
        for _i in 1..=steps {
            flashes += self.next_step();
            println!("After {} steps:\n{}", _i, self);
        }
        flashes
    }

    fn all_flash(&mut self) -> Option<usize> {
        let num_octs = self.grid.iter().flatten().count();
        for i in 1.. {
            if self.next_step() == num_octs {
                println!("After {} steps:\n{}", i, self);
                return Some(i);
            }
        }
        None
    }

    fn next_step(&mut self) -> usize {
        let mut flash_set: FlashSet = HashSet::new();
        // Increase all energy levels by 1
        for oct in self.grid.iter_mut().flatten() {
            *oct += 1;
        }
        // Flash any octopus with a value greater than 9
        for r in 0..self.grid.len() {
            for c in 0..self.grid[r].len() {
                if self.grid[r][c] > 9 && flash_set.insert((r,c)) {
                    self.flash(&mut flash_set, r, c);
                }
            }
        }
        for (r, c) in flash_set.iter() {
            self.grid[*r][*c] = 0;
        }
        flash_set.iter().count()
    }

    fn flash(&mut self, flash_set: &mut FlashSet, br: usize, bc: usize) {
        for r in usize_range(br, self.grid.len()-1) {
            for c in usize_range(bc, self.grid[0].len()-1) {
                if r == br && c == bc {
                    continue;
                }
                self.grid[r][c] += 1;
                if self.grid[r][c] > 9 && flash_set.insert((r,c)) {
                    self.flash(flash_set, r, c);
                }
            }
        }
    }
}

impl fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.grid {
            for c in r {
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn usize_range(u: usize, max: usize) -> impl Iterator<Item=usize> {
    if u > 0 && u < max {
        u-1..=u+1
    } else if u > 0 {
        u-1..=u
    } else {
        u..=u+1
    }
}

fn neighbor_iter<T>(grid: &Vec<Vec<T>>, r: usize, c: usize) -> impl Iterator<Item=(usize, usize)> {
    let row = usize_range(r, grid.len()-1);
    let col = usize_range(c, grid[0].len()-1);
    row.zip(col).filter(move |(ir,ic)| !(*ir == r && *ic == c))
}

fn count_flashes<B: io::BufRead>(bufread: B, steps: usize) -> usize {
    let mut grid = OctopusGrid::new(bufread);
    grid.step_num(steps)
}

fn all_flash_at<B: io::BufRead>(bufread: B) -> usize {
    let mut grid = OctopusGrid::new(bufread);
    grid.all_flash().expect("No Possible Point at Which All Flash?")
}

#[cfg(test)]
mod test_flashes {
    use crate::count_flashes;
    use crate::all_flash_at;

    const SMALL: &[u8] = b"\
11111\n\
19991\n\
19191\n\
19991\n\
11111";

    const LARGE: &[u8] = b"\
5483143223\n\
2745854711\n\
5264556173\n\
6141336146\n\
6357385478\n\
4167524645\n\
2176841721\n\
6882881134\n\
4846848554\n\
5283751526";

    #[test]
    fn small_2() {
        assert_eq!(count_flashes(SMALL, 2), 9);
    }

    #[test]
    fn large_10() {
        assert_eq!(count_flashes(LARGE, 10), 204);
    }

    #[test]
    fn large_100() {
        assert_eq!(count_flashes(LARGE, 100), 1656);
    }

    #[test]
    fn large_all_flash() {
        assert_eq!(all_flash_at(LARGE), 195);
    }
}

fn main() -> io::Result<()>{
    let mut stdin = Vec::new();
    io::stdin().read_to_end(&mut stdin)?;
    println!("Flashes: {}", count_flashes(&stdin[..], 100));
    println!("All Flash At: {}", all_flash_at(&stdin[..]));
    Ok(())
}
