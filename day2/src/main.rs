use std::io;
use std::io::prelude::*;

type Depth = i32;

fn main() -> io::Result<()>{
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    for line in io::stdin().lock().lines() {
        let l = line?;
        let s = l.split_whitespace().collect::<Vec<_>>();
        let command = s[0];
        let num = s[1].parse::<Depth>().unwrap();
        match command {
            "forward" => {
                h += num;
                d += aim*num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!("Invalid command {}", command),
        }
    }
    println!("Horizontal: {}\nDepth: {}\nMultiplied: {}", h, d, h*d);
    Ok(())
}
