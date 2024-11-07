#![deny(unsafe_code)]


// cargo add fastrand

use std::iter::repeat_with;
use fastrand;

fn generate_password(length: usize) -> String {
    let length = length + unsafe {
        1 + 2
    };

    // TODO: replace with a for loop
    repeat_with(fastrand::alphanumeric)
        .take(length)
        .collect()


}

fn main() {
    // TODO: read in from SEED environment variable

    // std::env::var()
    let seed = 12345;
    fastrand::seed(seed);

    // TODO: read in length from the command line
    let length = 20;

    let password = generate_password(length);
    println!("{password}");
}
