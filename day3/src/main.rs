use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()>{
    let lines = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    let (zero, one) = count_bits(&lines);
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

    let oxygen_num = rating(&mut oxygen, '0', '1');
    let co2_num = rating(&mut co2, '1', '0');
    println!("Oxygen: {}\nCO2: {}\nMultiplied: {}",
             oxygen_num, co2_num, oxygen_num*co2_num);
    Ok(())
}

fn bin(n: &str) -> usize {
    usize::from_str_radix(n, 2).unwrap()
}

fn rating(vec: &mut Vec<String>, z: char, o: char) -> usize {
    for i in 0..vec[0].len() {
        let (zero, one) = count_bits_at(vec, i);
        if zero > one {
            vec.retain(|n| get_char(n, i) == z);
        } else {
            vec.retain(|n| get_char(n, i) == o);
        }
        if vec.len() == 1 {
            return bin(&vec[0]);
        }
    }
    panic!("Could not find rating!")
}

fn get_char(s: &String, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

fn count_bits(lines: &[String]) -> (Vec<usize>, Vec<usize>) {
    let mut zero = Vec::<usize>::new();
    let mut one = Vec::<usize>::new();
    for line in lines {
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
    (zero, one)
}

fn count_bits_at(lines: &[String], i: usize) -> (usize, usize) {
    let mut zero = 0;
    let mut one = 0;
    for line in lines {
        match line.chars().nth(i).unwrap() {
            '0' => zero += 1,
            '1' => one += 1,
            c => panic!("Invalid bit: '{}'", c),
        }
    }
    (zero, one)
}
