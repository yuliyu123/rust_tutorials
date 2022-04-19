use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()>{
    let path = "filt.txt";

    let mut file = File::create(path)?;
    write!(file, "hello/n?/n").expect("write failed");

    let input = File::open(path)?;
    let buff = BufReader::new(input);

    for line in buff.lines().map(|x| x.unwrap()) {
        println!("string: {}", line)
    }
    Ok(())
}
