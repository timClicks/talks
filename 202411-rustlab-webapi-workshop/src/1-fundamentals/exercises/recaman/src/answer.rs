use std::collections::HashSet;

fn recaman_sequence(n: usize) -> Vec<i32> {
    let mut seen = HashSet::new();
    seen.insert(0);
    // Create a second collection, which is ordered
    let mut sequence = Vec::with_capacity(n);
    sequence.push(0);
    let mut prev = 0;

    for i in 1..=n {
        let next = prev - i as i32;

        if next > 0 && !seen.contains(&next) {
            prev = next;
        } else {
            prev = prev + i as i32;
        }
        seen.insert(prev);
        sequence.push(prev);
    }
    sequence
}

fn recaman_find_first_duplicate() -> (usize, i32) {
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut prev = 0;

    for i in 1..1000000 { // use a large number so that it terminates
        let next = prev - i as i32;

        if next > 0 && !seen.contains(&next) {
            prev = next;
        } else {
            prev = prev + i as i32;
        }

        if seen.contains(&prev) {
            return (i, prev);
        }

        seen.insert(prev);
    }

    (0, 0)
}

fn recaman_find_first_100() -> usize {
    let mut need: HashSet<i32> = HashSet::new();
    for n in 1..=100 {
        need.insert(n);
    }
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut prev = 0;

    for i in 1..1000000 { // use a large number so that it terminates
        let next = prev - i as i32;

        if next > 0 && !seen.contains(&next) {
            prev = next;
        } else {
            prev = prev + i as i32;
        }

        seen.insert(prev);
        let _ = need.remove(&prev);
        if need.is_empty() {
            return i;
        }
    }

    0
}

fn main() {
    // Task 1
    println!("{:?}", recaman_sequence(15));

    //Task 2
    println!("{:?}", recaman_find_first_duplicate());

    //Task 3
    println!("{:?}", recaman_find_first_100());
}