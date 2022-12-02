use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            println!("BLANK LINE FOUND");
        } else {
            println!("{}", line?);
        }
    }
    Ok(())
}
