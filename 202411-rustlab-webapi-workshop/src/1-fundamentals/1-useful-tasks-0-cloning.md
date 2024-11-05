<section class="slide">

# Cloning

</section>
<section class="slide">

Sometimes, especially when starting with with collections and `for` loops, you'll encounter errors about values being moved.
<span class="fragment">Cloning is a useful strategy to work around this problem while you're learning the language.</span>

</section>
<section class="slide">

```rust,editable
fn main() {
    let items = vec![1, 2, 3];

    for item in items {
        println!("{item}");
    }

    for item in items {
        println!("{item}");
    }
}
```

</section>