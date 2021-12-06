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
    fn new(days: usize, numfish: usize) -> Self {
        Lantern {
            days: days,
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

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let mut fish = line?.split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .map(|n| Lantern::new(n, 1))
            .collect::<Vec<Lantern>>();
        println!("Initial state: {:?}", fish);
        for i in 1..=DAYS {
            let mut newfish: usize = 0;
            for f in fish.iter_mut() {
                newfish += f.day();
            }
            if newfish > 0 {
                fish.push(Lantern::new(NEW, newfish));
            }
        }
        println!("Total after {} days: {}", DAYS, fish.iter().map(|f| f.numfish).sum::<usize>());
    }
    Ok(())
}
