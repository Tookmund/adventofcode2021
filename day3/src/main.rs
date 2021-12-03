use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()>{
    let mut zero = Vec::<usize>::new();
    let mut one = Vec::<usize>::new();
    for line in io::stdin().lock().lines() {
        for (i, c) in line?.chars().enumerate() {
            if i >= zero.len() {
                zero.push(0);
                one.push(0);
            }
            match c {
                '0' => zero[i] += 1,
                '1' => one[i] += 1,
                _ => panic!("Invalid bit: '{}'", c),
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..zero.len() {
        if zero[i] > one[i] {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }
    let gamma_num = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = usize::from_str_radix(&epsilon, 2).unwrap();
    println!("Gamma: {}\nEpsilon: {}\nMultiplied: {}",
             gamma_num, epsilon_num, gamma_num*epsilon_num);
    Ok(())
}
