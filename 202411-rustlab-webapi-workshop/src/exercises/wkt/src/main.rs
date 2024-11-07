

#[derive(Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// #[derive(Debug)]
// pub struct LineString {
//     points: Vec<Point>,
// }

// impl std::str::FromStr for LineString {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }


fn main() {
    let a = "POINT (123, 345)".parse::<Point>().unwrap();
    let b = "POINT (345, 123)".parse::<Point>().unwrap();

    println!("{a:?}");
    println!("{b:?}")

    // let c: LineString = "LINESTRING (30 10, 10 30, 40 40)".parse().unwrap();
    // println!("{b:?}");
}
