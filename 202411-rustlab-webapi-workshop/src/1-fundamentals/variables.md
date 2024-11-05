<section class="slide">

# Variables and mutability

</section>

<section class="slide">

Variables are defined with `let`:

</section>

<section class="slide">

```rust,editable
fn main() {
    let port: u16 = 8080;
    println!("port: {port}");
    // port = 80;
    println!("port: {port}");
}

// TODO: show - immutable by default
```

</section>

Rust provides its type safety via static typing<span class=fragment>,
but it's fairly uncommon to see type hints (<code>: u16</code>) in the body of functions because of extensive type inference.</span>.
