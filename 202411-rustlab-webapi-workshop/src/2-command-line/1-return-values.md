<section class="slide">

# Return values

</section>

<section class="slide">

A `main()` function in a Rust program does not return an integer.
Instead, it returns unit (`()`) by default.

```rust
fn main() /* -> () */ {
   // ...
}
```

</section>
<section class="slide">

Main can also return the `Result` type.
`Result` either contains an `Ok` value indicating success, or an `Err` value which indicates failure.

```rust
fn main() -> std::io::Result<()> {
   // ...
}
```

</section>