fn main() {
    let numbers: Vec<_> = std::env::args()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    todo!()
}
