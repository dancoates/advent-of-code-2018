use std::collections::HashMap;
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
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let mut frequencies = HashMap::new();
        for ch in line.chars() {
            let freq = frequencies.entry(ch).or_insert(0);
            *freq += 1;
        }

        let vals3 = frequencies.values().filter(|x| **x == 3);
        let vals2 = frequencies.values().filter(|x| **x == 2);

        if vals3.count() > 0 {
            threes += 1;
        }

        if vals2.count() > 0 {
            twos += 1;
        }
    }

    writeln!(io::stdout(), "{}", twos);
    writeln!(io::stdout(), "{}", threes);
    writeln!(io::stdout(), "{}", twos * threes);

    Ok(())
}
