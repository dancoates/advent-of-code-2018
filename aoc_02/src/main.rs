use std::collections::HashMap;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

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

    writeln!(io::stdout(), "part1: {}", twos * threes);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    for line1 in input.lines() {
        for line2 in input.lines() {
            let mut differences = 0;
            let mut same = String::from("");

            let mut line2_chars = line2.chars();

            for ch1 in line1.chars() {
                let ch2 = line2_chars.next().unwrap();
                if ch1 != ch2 {
                    differences += 1;
                } else {
                    same.push(ch1);
                }
            }

            if differences == 1 {
                writeln!(io::stdout(), "part2: {}", same);
                return Ok(());
            }
        }
    }
    Ok(())
}
