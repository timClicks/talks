
// use std::ops::Deref;

// struct Rabbit;

// impl Rabbit {
//     fn hop(&self) {
//         println!("magic!")
//     }
// }

// struct MagicWand;

// impl Deref for MagicWand {
//     type Target = Rabbit;

//     fn deref(&self) -> &Self::Target {
//         &Rabbit
//     }
// }


// fn main() {
//     let magic_wand = MagicWand;

//     magic_wand.hop()
// }




#[derive(Debug)]
struct ItalianCity {
    name: String,
    region: String,
    population_2011: u32,
    population_2021: u32,
}

#[must_use]
enum Trial<P, F> {
    Pass(P),
    Failure(F),
}

fn main() {
    let prato = ItalianCity {
        name: "Prato".to_string(),
        region: "Tuscany".to_string(),
        population_2011: 185456,
        population_2021: 200762,
    };

    // match prato {
    //     ItalianCity { region, name: city_name, population_2011: 0..=500_000, .. } => { },
    //     ItalianCity { name: city_name, population_2011: 500_001.., ..} => { },
    //     // ItalianCity { region: "Lazio", name: city_name, .. } => { },
    // }
    
    let cities = vec![prato];
    
    let small_cities = cities.iter()
        .filter(|c|{ c.population_2021 < 500_000 });
    // for city in small_cities {
    //     let ItalianCity { region, name, .. } = city;
    //     println!("{}, {}", city.name, city.region);
    // }
    for city in small_cities {
        println!("{region}, {name}", name=city.name);
    }
    // let _ = Trial::<i32, String>::Failure("something went wrong".to_string());
    // // std::fs::read_to_string("italian-cities.csv")

    // let items = Vec::new();
    
    // for i in 0..100 {
    //     items.push(i);
    // }

    // let name = "Tim";
    // println!("Hello, {}!", name)
}



// https://play.rust-lang.org

// type ErrorMessage = String;

// struct Result(i32, Option<ErrorMessage>);

// struct Outcome<T, E> {
//     ok: T,
//     err: Option<E>,
// }

#[must_use]
enum Trial<P, F> {
    Pass(P),
    Failure(F),
}

fn main() {
    Trial::<i32, String>::Pass(99);

    // let _ = Trial::<i32, String>::Failure("something went wrong".to_string());


    // // std::fs::read_to_string("italian-cities.csv")

    // let items = Vec::new();
    
    // for i in 0..100 {
    //     items.push(i);
    // }

    // let name = "Tim";
    // println!("Hello, {}!", name)
}
