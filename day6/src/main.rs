use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Lantern {
    days: usize,
}

const RESET: usize = 6;
const NEW: usize = 8;
const DAYS: usize = 256;

impl Lantern {
    fn new(days: usize) -> Self {
        Lantern {
            days: days
        }
    }
    fn day(&mut self) -> Option<Lantern> {
        if self.days == 0 {
            self.days = RESET;
            Some(Self::new(NEW))
        } else {
            self.days -= 1;
            None
        }
    }
}

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let mut fish = line?.split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .map(|n| Lantern::new(n))
            .collect::<Vec<Lantern>>();
        println!("Initial state: {:?}", fish);
        for i in 1..=DAYS {
            let mut newfish: Vec<Lantern> = Vec::new();
            for f in fish.iter_mut() {
                match f.day() {
                    Some(v) => newfish.push(v),
                    None => ()
                }
            }
            fish.extend(newfish);
            println!("Day {}", i);
        }
        println!("Total after {} days: {}", DAYS, fish.len());
    }
    Ok(())
}
