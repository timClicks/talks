// Adds env_logger (cargo add env_logger)
// Adds Actix Web (cargo add actix_web)
// Adds serde (cargo add serde --features=derive)
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use env_logger::Env;
use serde::Serialize;

use std::fs::File;
use std::io::BufReader;
use std::sync::LazyLock;

static DATABASE: LazyLock<ItalianCities> = LazyLock::new(|| {
    let data_source = "../italian-cities.csv";
    load_data(data_source).unwrap()
});

#[derive(Debug, Parser)]
struct Args {
    /// The name of the city to return information about
    city: String,
}

#[derive(Debug, Serialize)]
struct ItalianCity {
    name: String,
    region: String,
    population_2011: u32,
    population_2021: u32,
}

#[derive(Debug)]
struct ItalianCities {
    data: Vec<ItalianCity>,
}

impl ItalianCities {
    fn add(&mut self, city: ItalianCity) {
        self.data.push(city)
    }

    fn find(&self, name: &str) -> Option<&ItalianCity> {
        for city in &self.data {
            if city.name.to_lowercase() == name.to_lowercase() {
                return Some(&city);
            }
        }

        None
    }
}

fn load_data(data_source: &str) -> Result<ItalianCities, Box<dyn std::error::Error>> {
    let file = File::open(data_source)?;
    let reader = BufReader::new(file);
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
            population_2011: record
                .get(2)
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
            population_2021: record
                .get(3)
                .unwrap_or_default()
                .parse()
                .unwrap_or_default(),
        };
        cities.add(city);
    }

    Ok(cities)
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = Args::parse();
//     let cities = &DATABASE;

//     match cities.find(&args.city) {
//         Some(city) => println!("{:?}", city),
//         None => {
//             println!("Not found");
//             std::process::exit(1);
//         },
//     }

//     Ok(())
// }

#[get("/")]
async fn index() -> impl Responder {
    "Ciao, Firenze!"
}

#[get("/{city}")]
async fn lookup_city(city: web::Path<String>) -> impl Responder {

    match DATABASE.find(&city) {
        Some(info) => HttpResponse::Ok().json(info),
        None => HttpResponse::NotFound()
            .content_type("application/json")
            .body(format!(r#"{{"query":"{city}"}}"#))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(lookup_city)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
