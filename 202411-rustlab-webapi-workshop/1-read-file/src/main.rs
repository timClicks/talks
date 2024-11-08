use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("../italian-cities.csv")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    for (i, line) in contents.lines().enumerate() {
        if i > 5 {
            break;
        }
        print!("{i}: ");

        for field in line.split(',') {
            print!("{field} ");
        }

        println!()
    }

    Ok(())
}