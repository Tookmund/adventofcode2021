use std::io;
use std::io::prelude::*;

type Num = i32;

struct MinFuel {
    fuel: Num,
    position: Num
}

fn main() -> io::Result<()>{
    for line in io::stdin().lock().lines() {
        let crabs = line?.split(',')
            .map(|n| n.parse::<Num>().unwrap())
            .collect::<Vec<Num>>();
        let mut min_fuel: Option<MinFuel> = None;
        for t in *crabs.iter().min().unwrap()..*crabs.iter().max().unwrap() {
            let fuel = required_fuel(t, &crabs);
            min_fuel = match min_fuel {
                None => Some(MinFuel {
                    fuel: fuel,
                    position: t
                }),
                Some(v) => if v.fuel > fuel {
                    Some(MinFuel {
                        fuel: fuel,
                        position: t
                    })
                } else {
                    Some(v)
                }
            }
        }
        match min_fuel {
            None => println!("No Minimum Fuel!"),
            Some(v) => println!("Minimum Fuel: {}\nPosition: {}",
                                v.fuel, v.position),
        }
    }
    Ok(())
}

fn required_fuel(target: Num, crabs: &[Num]) -> Num {
    // Triangle Numbers, factorial but with addition
    // https://en.wikipedia.org/wiki/Triangular_number
    crabs.iter().map(|c| {
        let n = Num::abs(c-target);
        (n*(n+1))/2
    }).sum()
}
