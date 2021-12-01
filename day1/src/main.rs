use std::io;
use std::io::prelude::*;

type Depth = i32;

fn main() -> io::Result<()>{
    let nums = io::stdin().lock().lines()
        .map(|x| x.unwrap().parse::<Depth>().unwrap()).collect::<Vec<_>>();
    let mut inc = 0;
    let mut prev: Option<Depth> = None;
    for n in nums.windows(3) {
        let cur: Depth = n.iter().sum();
        match prev {
            None => (),
            Some(v) => {
                if cur > v {
                    inc += 1;
                }
            }
        }
        prev = Some(cur);
    }
    println!("Increases: {}", inc);
    Ok(())
}
