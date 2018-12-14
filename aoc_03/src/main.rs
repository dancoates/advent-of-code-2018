use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
// use nested hashmaps to store things
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    writeln!(io::stdout(), "part1: {}", 1);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    writeln!(io::stdout(), "part2: {}", 1);

    Ok(())
}
