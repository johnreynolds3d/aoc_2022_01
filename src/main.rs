use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut map = HashMap::new();

    let mut elf: u8 = 1;
    let mut sum: u32 = 0;
    let mut num: u32;

    for line in f.lines() {
        if line.as_ref().unwrap().is_empty() {
            map.insert(elf, sum);
            elf += 1;
            sum = 0;
        } else {
            num = line?.trim().parse()?;
            sum += num;
        }
    }
    map.insert(elf, sum);

    let mut max: u32 = 0;
    let mut win: u8 = 0;

    for (key, value) in &map {
        if value > &max {
            max = *value;
            win = *key;
        }
    }
    println!("And the winner is... elf #{} with {} calories!", win, max);

    Ok(())
}
