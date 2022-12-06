use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut n: u32;
    let mut s = 0;
    let mut v = Vec::new();

    for line in lines {
        if !line.as_ref().unwrap().is_empty() {
            n = line?.trim().parse()?;
            s += n;
        } else {
            v.push(s);
            s = 0;
        }
    }
    v.sort();

    let l = v.len();
    let t = v[l - 1] + v[l - 2] + v[l - 3];

    println!("Maximum calories held by a single elf: {}", v[l - 1]);
    println!("Calories held by top 3 elves: {}", t);

    Ok(())
}
