use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()>{
    let mut zero = Vec::<usize>::new();
    let mut one = Vec::<usize>::new();
    let lines = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
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
    let gamma_num = bin(&gamma);
    let epsilon_num = bin(&epsilon);
    println!("Gamma: {}\nEpsilon: {}\nMultiplied: {}",
             gamma_num, epsilon_num, gamma_num*epsilon_num);

    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    println!("Oxygen");
    let oxygen_num = rating(&zero, &one, &mut oxygen, '0', '1');
    println!("CO2");
    let co2_num = rating(&zero, &one, &mut co2, '1', '0');
    println!("Oxygen: {}\nCO2: {}\nMultiplied: {}",
             oxygen_num, co2_num, oxygen_num*co2_num);
    Ok(())
}

fn bin(n: &str) -> usize {
    usize::from_str_radix(n, 2).unwrap()
}

fn rating(zero: &[usize], one: &[usize], vec: &mut Vec<String>,
             z: char, o: char) -> usize {
    for i in 0..zero.len() {
        if zero[i] > one[i] {
            println!("0 at {}", i);
            vec.retain(|n| get_char(n, i) == z);
        } else {
            println!("1 at {}", i);
            vec.retain(|n| get_char(n, i) == o);
        }
        println!("{:?}", vec);
        if vec.len() == 1 {
            return bin(&vec[0]);
        }
    }
    panic!("Could not find rating!")
}

fn get_char(s: &String, i: usize) -> char {
    s.chars().nth(i).unwrap()
}
