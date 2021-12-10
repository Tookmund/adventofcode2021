use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct HeightMap {
    hm: Vec<Vec<usize>>
}

impl HeightMap {
    fn risk_levels(&self) -> usize {
        let mut ret = 0;
        for r in 0..self.hm.len() {
            for c in 0..self.hm[r].len() {
                match self.lowest_neighbor(r, c) {
                    None => ret += self.hm[r][c]+1,
                    Some(_) => (),
                }
            }
        }
        ret
    }

    fn update_lowest(&self, o: &Option<(usize, usize)>, r: usize, c: usize) -> Option<(usize, usize)> {
        match *o {
            None => Some((r,c)),
            Some((mr, mc)) => if self.hm[mr][mc] > self.hm[r][c] {
                Some((r,c))
            } else {
                Some((mr,mc))
            }
        }
    }

    fn lowest_neighbor(&self, r: usize, c: usize) -> Option<(usize, usize)> {
        let mut ret: Option<(usize, usize)> = None;

        if r > 0 && self.hm[r][c] >= self.hm[r-1][c] { // Top
            ret = self.update_lowest(&ret, r-1, c);
        }
        if c > 0 && self.hm[r][c] >= self.hm[r][c-1] { // Left
            ret = self.update_lowest(&ret, r, c-1);
        }
        if c < self.hm[0].len()-1 && self.hm[r][c] >= self.hm[r][c+1] { // Right
            ret = self.update_lowest(&ret, r, c+1);
        }
        if r < self.hm.len()-1 && self.hm[r][c] >= self.hm[r+1][c] { // Bottom
            ret = self.update_lowest(&ret, r+1, c);
        }
        ret
    }

    fn basin(&self, r: usize, c: usize) -> Option<(usize, usize)> {
        if self.hm[r][c] == 9 {
            None
        } else {
            match self.lowest_neighbor(r,c) {
                None => Some((r,c)),
                Some((r,c)) => self.basin(r,c),
            }
        }
    }

    fn three_largest_basins(&self) -> usize {
        let mut basin_map: HashMap<(usize, usize), usize> = HashMap::new();
        for r in 0..self.hm.len() {
            for c in 0..self.hm[0].len() {
                match self.basin(r, c) {
                    None => (),
                    Some(basin) => *basin_map.entry(basin).or_insert(0) += 1,
                }
            }
        }
        let mut basins = basin_map.iter().collect::<Vec<_>>();
        // Reverse sort, should be a cmp b
        basins.sort_unstable_by(|a, b| b.1.cmp(a.1));
        if basins.len() < 3 {
            0
        } else {
            basins[0].1*basins[1].1*basins[2].1
        }
    }
}

impl<'a> FromIterator<&'a str> for HeightMap {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        HeightMap {
            hm: iter.into_iter()
                .map(|l|
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>()
        }
    }
}

fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin().lock().lines().map(|s| s.unwrap()).collect();
    let hm: HeightMap = lines.iter().map(|s| s as &str).collect();
    println!("Risk Level: {}", hm.risk_levels());
    println!("Three Largest Basins: {}", hm.three_largest_basins());
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

    #[test]
    fn part2_example() {
        let hm: HeightMap = "\
2199943210\n\
3987894921\n\
9856789892\n\
8767896789\n\
9899965678".split_whitespace().collect();
        assert_eq!(hm.three_largest_basins(), 1134);
    }
}
