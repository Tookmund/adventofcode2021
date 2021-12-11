use std::io;
use std::io::prelude::*;

fn count_1_4_7_8<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    let mut count = 0;
    for line in bufread.lines() {
        let l = line?;
        let pipe = l.chars().position(|c| c == '|').expect("Missing | character!");
        count += l[pipe+1..].split_whitespace().filter(|s| match s.len() {
            2 | 4 | 3 | 7 => true,
            _ => false
        }).count()
    }
    Ok(count)
}

#[cfg(test)]
mod test {
    use crate::count_1_4_7_8;

    const EXAMPLE: &[u8] = b"\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |\
fdgacbe cefdb cefbgd gcbe\n\
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |\
fcgedb cgb dgebacf gc\n\
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |\
cg cg fdcagb cbg\n\
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |\
efabcd cedba gadfec cb\n\
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |\
gecf egdcabf bgf bfgea\n\
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |\
gebdcfa ecba ca fadegcb\n\
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |\
cefg dcbef fcge gbcadfe\n\
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |\
ed bcgafe cdgba cbgef\n\
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |\
gbdfcae bgc cg cgb\n\
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |\
fgae cfgab fg bagce";

    #[test]
    fn part1() {
        assert_eq!(count_1_4_7_8(EXAMPLE).unwrap(), 26)
    }

}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("Count 1, 4, 7, 8: {}", count_1_4_7_8(io::stdin().lock())?);
    Ok(())
}
