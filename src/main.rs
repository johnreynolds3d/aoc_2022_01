use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut num: u32;
    let mut sum: u32 = 0;
    let mut vec = Vec::new();

    for line in f.lines() {
        if !line.as_ref().unwrap().is_empty() {
            num = line?.trim().parse()?;
            sum += num;
        } else {
            vec.push(sum);
            sum = 0;
        }
    }
    vec.sort();

    let l = vec.len();
    let top3 = vec[l - 1] + vec[l - 2] + vec[l - 3];

    println!("Maximum calories held by a single elf: {}.", vec[l - 1]);
    println!("Calories held by top 3 elves: {}.", top3);

    Ok(())
}
