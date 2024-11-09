use actix_web::{get, web::{self, Path}, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use serde::{Serialize, Deserialize};
// use serde_json;

use std::sync::LazyLock;

static DATABASE: LazyLock<ItalianCities> = LazyLock::new(|| {
    let data_source = "../italian-cities.csv";
    load_data(data_source).unwrap()
});

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    name: Option<String>,
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

fn load_data(path: &str) -> Result<ItalianCities, Box<dyn std::error::Error>> {
    let city_data = std::fs::read_to_string(path);
    let city_data = city_data?;
    let reader = city_data.as_bytes();

    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut cities = ItalianCities { data: Vec::new() };

    // match csv_reader.headers() {
    //     Ok(headers) => {
    //         println!("Headers:");
    //         println!("{headers:?}");
    //     }
    //     _ => (),
    // }

    let records = csv_reader
        .records()
        .filter_map(|record| record.ok() )
        //.take(5)
        ;

    for record in records {
        let name: &str = record.get(1).unwrap_or_default();
        // println!("{name}");

        let population_2011_raw = record.get(2).unwrap_or_default();
        let population_2011: u32 = population_2011_raw.parse().unwrap_or_default();
        // println!("{population_2011}");

        let population_2021_raw = record.get(3).unwrap_or_default();
        let population_2021: u32 = population_2021_raw.parse().unwrap_or_default();
        // println!("{population_2021}");

        let city = ItalianCity {
            name: name.to_string(),
            region: record.get(5).unwrap_or_default().to_string(),
            population_2011,
            population_2021
        };

        cities.add(city);
    }

    Ok(cities)
}


// copied from actix.rs

#[get("/")]
async fn index() -> impl Responder {
    "Ciao, Firenze!"
}

#[get("/{city}")]
async fn hello(Path(name): Path<String>) -> impl Responder {
    match DATABASE.find(&name) {
        Some(city) => {
            // return HTTP 200
            // let population = city.population_2021.to_string();
            // HttpResponse::Ok().body(population)

            // let city_as_json = format!(r#"{{
            //     "name": "{name}",
            //     "population-2021": {pop2021}
            // }}"#, name=city.name, pop2021=city.population_2021);
            // HttpResponse::Ok()
            //     .content_type("application/json")
            //     .body(city_as_json)

            HttpResponse::Ok().json(city)
        },
        None => {
            // HTTP 404
            HttpResponse::NotFound().body("not found")
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_info = env_logger::Env::default()
        .default_filter_or("info");
    env_logger::init_from_env(env_info);

    HttpServer::new(|| App::new()
        .wrap(actix_web::middleware::Logger::default())
        .service(index)
        .service(hello)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let env_info = env_logger::Env::default()
//         .default_filter_or("info");
//     env_logger::init_from_env(env_info);

//     log::info!("starting up");

//     let args = Args::parse();

//     let cities = &DATABASE;
//     let name = args.name.unwrap_or("Rome".to_string());
//     match cities.find(&name) {
//         Some(city) => println!("{city:?}"),
//         None => {
//             eprintln!("not found");
//             std::process::exit(1);
//         },
//     }

//     // for city in cities.iter().take(5) {
//     //     println!("{city:?}")
//     // }
//     // question mark - shorthand for the code below:
//     //
//     // match city_data {
//     //     Ok(data) => println!("{data:?}"),
//     //     Err(err) => return Err(err),
//     // }

//     // for (i, line) in city_data.lines().enumerate() {
//     //     if i > 5 {
//     //         break;
//     //     }
//     //     print!("{i}: ");

//     //     for field in line.split(',') {
//     //         print!("{field} ");
//     //     }

//     //     println!()
//     // }

//     Ok(())
// }
