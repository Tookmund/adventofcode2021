use std::io;
use std::io::prelude::*;
use std::ops::RangeInclusive;

type Num = u32;

#[derive(Debug)]
struct HeightMap {
    hm: Vec<Vec<Num>>
}

fn _get_range(n: usize, m: usize) -> RangeInclusive<usize> {
    if n > 0 && n < m {
        n-1..=n+1
    } else if n > 0 {
        n-1..=n
    } else if n < m {
        n..=n+1
    } else {
        n-1..=n+1
    }
}

impl HeightMap {
    fn risk_levels(&self) -> Num {
        let mut ret = 0;
        for r in 0..self.hm.len() {
            for c in 0..self.hm[r].len() {
                if self.low_point(r, c) {
                    ret += self.hm[r][c]+1;
                }
            }
        }
        ret
    }

    fn low_point(&self, r: usize, c: usize) -> bool {
        if r > 0 && self.hm[r][c] >= self.hm[r-1][c] { // Top
            false
        } else if c > 0 && self.hm[r][c] >= self.hm[r][c-1] { // Left
            false
        } else if c < self.hm[0].len()-1 && self.hm[r][c] >= self.hm[r][c+1] { // Right
            false
        } else if r < self.hm.len()-1 && self.hm[r][c] >= self.hm[r+1][c] { // Bottom
            false
        } else {
            true
        }
    }
}

impl FromIterator<Result<String, std::io::Error>> for HeightMap {
    fn from_iter<I: IntoIterator<Item = Result<String, std::io::Error>>>(iter: I) -> Self {
        HeightMap {
            hm: iter.into_iter()
                .map(|l|
                    l.unwrap().chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<Num>>())
                .collect::<Vec<Vec<Num>>>()
        }
    }
}

impl<'a> FromIterator<&'a str> for HeightMap {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        HeightMap {
            hm: iter.into_iter()
                .map(|l|
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<Num>>())
                .collect::<Vec<Vec<Num>>>()
        }
    }
}

fn main() -> io::Result<()> {
    let hm: HeightMap = io::stdin().lock().lines().collect();
    println!("Risk Level: {}", hm.risk_levels());
    Ok(())
}


#[cfg(test)]
mod test {
    use crate::HeightMap;

    #[test]
    fn part1_example() {
        let hm: HeightMap = "\
2199943210\n\
3987894921\n\
9856789892\n\
8767896789\n\
9899965678".split_whitespace().collect();
        assert_eq!(hm.risk_levels(), 15);
    }
}
