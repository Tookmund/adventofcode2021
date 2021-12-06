use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::repeat;

type Num = i32;
type Coordinate = (Num, Num);

fn coord_range(coord1: Coordinate, coord2: Coordinate) -> Box<dyn Iterator<Item = Coordinate>> {
    if coord1.0 == coord2.0 {
        Box::new(Iterator::zip(
            repeat(coord1.0),
            num_range(coord1.1, coord2.1)
            ))
    } else if coord1.1 == coord2.1 {
        Box::new(Iterator::zip(
            num_range(coord1.0, coord2.0),
            repeat(coord1.1)
            ))
    } else {
        Box::new(Iterator::zip(
            num_range(coord1.0, coord2.0),
            num_range(coord1.1, coord2.1),
            ))
    }
}

fn num_range(num1: Num, num2: Num) -> Box<dyn Iterator<Item = Num>> {
    if num1 < num2 {
        Box::new(num1..=num2)
    } else {
        Box::new((num2..=num1).rev())
    }
}

struct Vents {
    map: HashMap<Coordinate, usize>,
    max: Num,
}

impl Vents {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            max: 0,
        }
    }
    fn add_line(&mut self, coord1: Coordinate, coord2: Coordinate) {
        for (x, y) in coord_range(coord1, coord2) {
            match self.map.get_mut(&(x,y)) {
                Some(v) => *v += 1,
                None => {
                    self.map.insert((x,y), 1);
                    ()
                },
            }
            if x > self.max {
                self.max = x;
            } else if y > self.max {
                self.max = y;
            }
        }
    }
    fn from_buf_read(buf_read: &mut dyn BufRead) -> Self {
        let mut vent = Self::new();
        for line in buf_read.lines() {
            let line = line.unwrap();
            let coordlist = line.split(" -> ")
                .map(|c| {
                    let mut t = c.split(',').map(|n| n.parse::<i32>().unwrap());
                    (t.next().unwrap(), t.next().unwrap())
                })
                .collect::<Vec<_>>();
            vent.add_line(coordlist[0], coordlist[1]);
        }
        vent
    }
    fn sum_overlaps(&self) -> usize {
        self.map.iter().filter(|(_,v)| **v > 1).count()
    }
}

impl fmt::Display for Vents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.max {
            for x in 0..=self.max {
                match self.map.get(&(x, y)) {
                    Some(v) => write!(f, "{}", v)?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn main() -> io::Result<()>{
    let vents = Vents::from_buf_read(&mut io::stdin().lock());
    println!("{}", vents);
    println!("Number of Overlaps: {}", vents.sum_overlaps());
    Ok(())
}
