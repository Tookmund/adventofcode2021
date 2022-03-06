use std::io;
use std::io::prelude::*;

fn function<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::function;

    macro_rules! make_tests {
        ( $func:expr, $( $name:ident, $input:expr, $output:expr ), +) => {
            $(
                #[test]
                fn $name () {
                    assert_eq!($func($input.as_bytes()).unwrap(), $output);
                }
             )*
        }
    }

    macro_rules! make_tests_method {
        ( $func:expr, $( $name:ident, $input:expr, $method:ident, $output:expr ), +) => {
            $(
                #[test]
                fn $name () {
                    assert_eq!($func($input.as_bytes()).unwrap().$method(), $output);
                }
             )*
        }
    }

    make_tests! {
        function,
            example_name, "EXAMPLE DATA", 0 //EXAMPLE RESULT
    }
}

fn main() -> io::Result<()>{
    //let mut stdin = Vec::new();
    //io::stdin().read_to_end(&mut stdin)?;
    println!("Function: {}", function(io::stdin().lock())?);
    Ok(())
}
