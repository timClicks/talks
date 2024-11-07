use image::open;

fn main() {
    let mut args = std::env::args();

    let filename = args.nth(1).expect("filename required");

    let img = open(filename).unwrap();

    // let thumbnail = img.resize(/* TODO: figure how how to get this to compile */);

    let thumbnail = img.resize(64, 64, image::imageops::CatmullRom);
    thumbnail.save("output.jpg").unwrap();
}

// Recommended steps:
//
// - https://crates.io/crates/image
// - look for the documentation link
// - use the search bar to find the method
