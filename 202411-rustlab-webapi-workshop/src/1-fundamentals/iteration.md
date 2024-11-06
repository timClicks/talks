# Iteration

<section class="slide">

## For loops

</section>

<section class="slide">

```rust,editable
fn main() {
    let items = vec![1, 2, 3, 4, 5];

    for item in items {
        println!("{item}");
    }
}
```

</section>
<section class="slide">

## Ranges

</section>

<section class="slide">

```rust,editable
fn main() {
    for n in 0..10 {
        println!("{item}");
    }
}

// TODO: show - difference between `0..10` and `0..=10`
// TODO: show - infinite ranges (`0..`)
```

</section>

<section class="slide">
</section>

<!-- <section class="slide">

Iteration works with the following concepts from the language:

- `std::iter::Iter`
- `std::iter::IterMut`
- `std::iter::IntoIter`
- ownership

</section> -->