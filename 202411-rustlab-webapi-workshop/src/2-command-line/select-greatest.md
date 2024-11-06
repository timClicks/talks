<section class="slide">

# Exercise: Select Greatest

</section>
<section class="slide">

Task: print out the greatest number from a list of arguments.
Here is some starter code:

```rust
{{ #include examples/greatest/src/main.rs }}
```

```rust
fn main() {
    let numbers: Vec<_> = std::env::args()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    todo!()
}
```

Hints:

- use the standard library's documentation (`rustup doc --std` opens it locally)

</section>