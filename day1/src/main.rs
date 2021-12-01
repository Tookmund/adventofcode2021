use std::io;
use std::io::prelude::*;

type Depth = i32;

fn main() -> io::Result<()>{
    let nums = io::stdin().lock().lines()
        .map(|x| x.unwrap().parse::<Depth>().unwrap()).collect::<Vec<_>>();
    let mut prev: Option<Depth> = None;
    let mut inc = 0;
    println!("{:?}", nums);
    for n in nums {
        match prev {
            None => prev = Some(n),
            Some(v) => {
                println!("{} to {}", v, n);
                if n > v {
                    inc += 1;
                }
                prev = Some(n);
            },
        };
    }
    println!("Increases: {}", inc);
    Ok(())
}
