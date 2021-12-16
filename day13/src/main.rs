use std::io;
use std::io::prelude::*;
use std::collections::HashSet;
use std::fmt;

struct Paper {
    dots: HashSet<(usize,usize)>,
    x_max: usize,
    y_max: usize,
}

enum Fold {
    X,
    Y,
}

impl Paper {
    fn new() -> Self {
        Paper {
            dots: HashSet::new(),
            x_max: 0,
            y_max: 0,
        }
    }

    fn mark(&mut self, x: usize, y: usize) -> bool {
        self.x_max = if x > self.x_max {
            x
        } else {
            self.x_max
        };
        self.y_max = if y > self.y_max {
            y
        } else {
            self.y_max
        };
        self.dots.insert((x,y))
    }

    fn fold(&mut self, f: Fold, point: usize) {
        match f {
            Fold::Y => {
                if point > self.y_max {
                    return
                }
                for y in point..=self.y_max {
                    for x in 0..=self.x_max {
                        if self.dots.remove(&(x,y)) {
                            self.dots.insert((x,self.y_max-y));
                        }
                    }
                }
                self.y_max -= point+1;
            },
            Fold::X => {
                if point > self.x_max {
                    return
                }
                for x in point..=self.x_max {
                    for y in 0..=self.y_max {
                        if self.dots.remove(&(x,y)) {
                            self.dots.insert((self.x_max-x,y));
                        }
                    }
                }
                self.x_max -= point+1;
            },
        }
    }
    fn count(&self) -> usize {
        self.dots.iter().count()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                match self.dots.get(&(x,y)) {
                    Some(_) => write!(f, "#")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

enum PState {
    Points,
    Folds,
}
fn instructions<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    let mut state = PState::Points;
    let mut paper = Paper::new();
    for line in bufread.lines() {
        let l = line?;
        if l.is_empty() {
            state = PState::Folds;
        } else {
            match state {
                PState::Points => {
                    let mut num = l.split(',').map(|n| n.parse::<usize>().expect("Points are not numbers?"));
                    paper.mark(num.next().unwrap(), num.next().unwrap());
                },
                PState::Folds => {
                    let vf = l.split_whitespace().collect::<Vec<_>>();
                    let f = vf[2];
                    paper.fold(
                        match f.chars().nth(0).unwrap() {
                            'y' => Fold::Y,
                            'x' => Fold::X,
                            c => panic!("fold along {} not x or y!", c),
                        },
                        f[2..].parse::<usize>().expect("Fold not a number?"));
                        println!("{}", paper);
                },
            }
        }
    }
    Ok(paper.count())
}

#[cfg(test)]
mod test {
    use crate::instructions;

    const EXAMPLE: &[u8] = b"\
6,10\n\
0,14\n\
9,10\n\
0,3\n\
10,4\n\
4,11\n\
6,0\n\
6,12\n\
4,1\n\
0,13\n\
10,12\n\
3,4\n\
3,0\n\
8,4\n\
1,10\n\
2,14\n\
8,10\n\
9,0\n\
\n\
fold along y=7\n\
fold along x=5";

    #[test]
    fn test_example() {
        assert_eq!(instructions(EXAMPLE).unwrap(), 16)
    }

}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("Number of Dots at the End: {}", instructions(io::stdin().lock())?);
    Ok(())
}
