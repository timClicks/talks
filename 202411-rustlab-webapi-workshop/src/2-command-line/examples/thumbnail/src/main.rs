fn main() {
    let mut args = std::env::args();

    let filename = args.nth(1).expect("filename required");

    let img = image::open(filename).unwrap();

    let thumbnail = img.resize(/* TODO: figure how how to get this to compile */);

    thumbnail.save("output.jpg").unwrap();
}
