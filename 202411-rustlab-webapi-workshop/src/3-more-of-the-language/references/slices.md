<section class="slide">

# Slices

</section>
<section class="slide">

A slice gives you a view into a larger collection:

```rust,editable
fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}
```

</section>