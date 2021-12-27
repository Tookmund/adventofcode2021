use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Num = usize;
type Coord = (usize,usize);
type Weights = Vec<Vec<Num>>;
type Distance = HashMap<Coord, Option<usize>>;
type Visited = HashSet<Coord>;

struct Cavern {
    weight: Weights,
    distance: Distance,
    visited: Visited,
}

impl Cavern {
    fn new<B: io::BufRead>(bufread: B) -> Self {
        let mut cavern = Cavern {
            weight: Weights::new(),
            visited: Visited::new(),
            distance: Distance::new(),
        };
        for (r, line) in bufread.lines().enumerate() {
            let l: Vec<Num> = line.unwrap().chars().map(|ch| ch.to_digit(10).expect("not a number") as usize).collect();
            cavern.weight.push(l);
        }
        cavern
    }
    fn update(&mut self, cur: &Coord, next: &Coord) -> bool {
        let curval = self.distance.get(cur).expect("No current value entry").expect("Missing current value!");
        let mut nextentry = self.distance.entry(*next).or_insert(None);
        let path = curval + self.weight[next.0][next.1];
        let ret = match self.visited.get(next) {
            None => true,
            Some(_) => false,
        };
        *nextentry = match nextentry {
            None => Some(path),
            Some(v) => if path > *v {
                Some(*v)
            } else {
                Some(path)
            },
        };
        ret
    }
    fn lowest(&self, lowest: &Option<Coord>, next: &Coord) -> Option<Coord> {
        Some(match *lowest {
            None => *next,
            Some(l) => if self.distance.get(next).unwrap().unwrap() < self.distance.get(&l).unwrap().unwrap() {
                *next
            } else {
                l
            },
        })
    }
    fn lowest_risk(&mut self, mut cur: Coord) -> usize {
        self.distance.insert(cur, Some(0));
        for r in 0..self.weight.len() {
            for c in 0..self.weight[r].len() {
                let cur = (r,c);
                self.visited.insert(cur);
                let mut lowest: Option<Coord> = None;
                if cur.0 > 0 {
                    let next = (cur.0-1, cur.1);
                    if self.update(&cur, &next) {
                        lowest = self.lowest(&lowest, &next);
                    }
                }
                if cur.0 < self.weight.len()-1 {
                    let next = (cur.0+1, cur.1);
                    if self.update(&cur, &next) {
                        lowest = self.lowest(&lowest, &next);
                    }
                }
                if cur.1 > 0 {
                    let next = (cur.0, cur.1-1);
                    if self.update(&cur, &next) {
                        lowest = self.lowest(&lowest, &next);
                    }
                }
                if cur.1 < self.weight[0].len()-1 {
                    let next = (cur.0, cur.1+1);
                    if self.update(&cur, &next) {
                        lowest = self.lowest(&lowest, &next);
                    }
                }
                //cur = lowest.expect("No Lowest Value!");
                println!("Current: {:?}", cur);
            }
        }
        self.distance.get(&(self.weight.len()-1, self.weight[0].len()-1)).unwrap().unwrap()
    }
}

fn lowest_risk<B: io::BufRead>(bufread: B) -> usize {
    let mut cavern = Cavern::new(bufread);
    cavern.lowest_risk((0,0))
}

#[cfg(test)]
mod test {
    use crate::lowest_risk;

    const EXAMPLE: &[u8] = b"\
1163751742\n\
1381373672\n\
2136511328\n\
3694931569\n\
7463417111\n\
1319128137\n\
1359912421\n\
3125421639\n\
1293138521\n\
2311944581";

    #[test]
    fn test_example() {
        assert_eq!(lowest_risk(EXAMPLE), 40)
    }

}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("Lowest Risk: {}", lowest_risk(io::stdin().lock()));
    Ok(())
}
