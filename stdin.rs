use std::io;
use std::io::prelude::*;

fn function<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::function;

    const EXAMPLE: &[u8] = b"\
TEST\n\
Data\n\
Here\n";

    #[test]
    fn test_example() {
        assert_eq!(function(EXAMPLE).unwrap(), 0)
    }

}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("Function: {}", function(io::stdin().lock())?);
    Ok(())
}
