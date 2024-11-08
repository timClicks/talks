// Adds clap (cargo add clap features=derive,wrap_help)
//
// Creates Args struct
use clap::Parser;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    /// The name of the city to return information about
    city: String,
}

#[derive(Debug)]
struct ItalianCity {
    name: String,
    region: String,
    population_2011: u32,
    population_2021: u32,
}

#[derive(Debug)]
struct ItalianCities {
    data: Vec<ItalianCity>
}

impl ItalianCities {
    fn add(&mut self, city: ItalianCity) {
        self.data.push(city)
    }

    fn find(&self, name: &str) -> Option<&ItalianCity> {
        for city in &self.data {
            if city.name.to_lowercase() == name.to_lowercase() {
                return Some(&city)
            }
        }

        None
    }
}

fn load_data(data_source: &str) -> Result<ItalianCities, Box<dyn std::error::Error>> {
    let file = File::open(data_source)?;
    let mut reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(reader);

    let mut cities = ItalianCities { data: vec![] };

    for (i, result) in reader.records().enumerate() {
        if i > 5 {
            break;
        }

        let record = result?;

        let city = ItalianCity {
            name: record.get(1).unwrap().to_string(),
            region: record.get(5).unwrap_or_default().to_string(),
            population_2011: record.get(2).unwrap_or_default().parse().unwrap_or_default(),
            population_2021: record.get(3).unwrap_or_default().parse().unwrap_or_default(),
        };
        cities.add(city);
    }

    Ok(cities)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data_source = "../italian-cities.csv";
    let cities = load_data(data_source)?;

    match cities.find(&args.city) {
        Some(city) => println!("{:?}", city),
        None => {
            println!("Not found");
            std::process::exit(1);
        },
    }

    Ok(())
}
