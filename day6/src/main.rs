use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Lantern {
    days: usize,
    numfish: usize,
}

const RESET: usize = 6;
const NEW: usize = 8;
const DAYS: usize = 256;

impl Lantern {

    fn initial_days(days: usize) -> Self {
        Lantern {
            days: days,
            numfish: 1,
        }
    }
    fn new_fish(numfish: usize) -> Self {
        Lantern {
            days: NEW,
            numfish: numfish,
        }
    }
    fn day(&mut self) -> usize {
        if self.days == 0 {
            self.days = RESET;
            self.numfish
        } else {
            self.days -= 1;
            0
        }
    }
}

fn lanterns_from_string(s: &str) -> Vec<Lantern> {
    s.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .map(|n| Lantern::initial_days(n))
        .collect::<Vec<Lantern>>()
}

fn spawn_lanterns(s: &str, days: usize) -> usize {
    let mut fish = lanterns_from_string(s);
    for _i in 1..=days {
        let mut newfish: usize = 0;
        for f in fish.iter_mut() {
            newfish += f.day();
        }
        if newfish > 0 {
            fish.push(Lantern::new_fish(newfish));
        }
    }
    fish.iter().map(|f| f.numfish).sum::<usize>()
}

#[cfg(test)]
mod test {
    use crate::spawn_lanterns;

    #[test]
    fn basic() {
        assert_eq!(spawn_lanterns("3", 5), 2);
    }

    #[test]
    fn part1_18() {
        assert_eq!(spawn_lanterns("3,4,3,1,2", 18), 26);
    }

    #[test]
    fn part1_80() {
        assert_eq!(spawn_lanterns("3,4,3,1,2", 80), 5934);
    }

    #[test]
    fn part2() {
        assert_eq!(spawn_lanterns("3,4,3,1,2", 256), 26984457539);
    }
}

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        println!("Total after {} days: {}", DAYS, spawn_lanterns(&line?, DAYS));
    }
    Ok(())
}
