// This example demonstrates that multiplication of arbitrary integers can cause panics.

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(not(kani))]
fn main() {
    let _ = multiply(i32::MAX, 10);
}

#[cfg(kani)]
mod verification {
    use crate::multiply;

    #[kani::proof]
    fn verify_can_multiply() {
        let a: i32 = kani::any();
        let b: i32 = kani::any();

        let _ = multiply(a, b);
    }
}