use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut frequencies = [0u8; 256];
    for line in input.lines() {
        let mut chars: Vec<char> = line.chars().collect();
        chars.sort();
        let sorted: String = chars.into_iter().collect();

        writeln!(io::stdout(), "{}", sorted);
    }

    Ok(())
}
