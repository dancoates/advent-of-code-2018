use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("./input.txt")?;
    let reader = BufReader::new(f);
    let mut counter = 0;
    for line in reader.lines() {
        let string = line.unwrap();
        let strnum: String = string.chars().skip(1).collect();
        let num = strnum.parse::<i32>().unwrap();
        let sign: String = string.chars().take(1).collect();
        let minus = "-";
        if sign == minus {
            counter -= num;
        } else {
            counter += num;
        }
    }

    println!("{}", counter);
    Ok(())
}
