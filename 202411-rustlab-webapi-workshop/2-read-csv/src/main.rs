use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../italian-cities.csv")?;
    let mut reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(reader);

    match reader.headers() {
        Ok(headers) => {
            println!("Headers:");
            println!("{headers:?}");
        },
        _ => (),
    }

    println!("Records:");
    for (i, result) in reader.records().enumerate() {
        if i > 5 {
            break;
        }
        print!("{i}: ");

        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}