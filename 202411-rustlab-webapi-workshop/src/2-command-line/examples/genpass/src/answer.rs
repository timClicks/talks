use std::iter::repeat_with;
use fastrand;

fn generate_password(length: usize) -> String {
    repeat_with(fastrand::alphanumeric)
        .take(length)
        .collect()
}

fn seed_from_systemtime() -> u64 {
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    duration.as_millis() as u64
}

fn main() {
    let s = "SEED";
    println!("{s:p}");

    let seed = std::env::var(s)
        .unwrap()
        .parse::<u64>()
        .unwrap_or(1234);
    fastrand::seed(seed);

    let mut args = std::env::args();
        let _ = args.next();
        // args.skip(1);

    let mut l = String::from("20");
    let length_raw = args
        .next()
        .unwrap_or(l.clone());
    println!("{:p}", l.clone().as_mut_ptr());
    println!("{:p}", l.clone().as_mut_ptr());

    let length = length_raw.parse::<usize>().unwrap();

    let password = generate_password(length);
    println!("{password}");
}
