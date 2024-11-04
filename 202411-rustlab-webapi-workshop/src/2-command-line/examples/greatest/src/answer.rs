fn main() {
    let mut numbers: Vec<_> = std::env::args()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap()) // Idea: consider f32
        .collect();

    // Idea: return early
    if numbers.is_empty() {
        return;
    }

    numbers.sort();

    // Idea: reverse and select the first element
    //
    // numbers.reverse();
    // if let Some(n) =  numbers.get(0) {
    //     println!("{n}")
    // }

    // SAFETY: numbers is not empty, therefore has a length of at least 1
    let n = numbers[numbers.len()-1];
    println!("{n}");
}