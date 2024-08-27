#[derive(Debug, Clone, Copy)]
enum Penguin {
   Hoiho, Kororā, Tawaki,
}

fn main() {
	let sighting = Penguin::Kororā;
	let another_sighting = sighting;
	println!("{sighting:?}");
}
