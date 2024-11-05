fn recaman(n: usize) -> i32 {
    let mut seen = std::collections::HashSet::new();
    seen.insert(0);
    let mut prev = 0;

    for i in 1..=n {
        let next = prev - i as i32;

        if next > 0 && !seen.contains(&next) {
            prev = next;
        } else {
            prev = prev + i as i32;
        }
        seen.insert(prev);
    }
    prev
}

fn main() {
    println!("{}", recaman(10));
}