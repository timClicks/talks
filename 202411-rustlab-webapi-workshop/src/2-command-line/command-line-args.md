<section class="slide">

# Command-line arguments

</section>
<section class="slide">

We'll see how this works manually (for now).

</section>
<section class="slide">

Use the `args` function `std::env` module to access the command-line arguments.
<span class="fragment">Note: this returns an iterator.</span>

```rust,editable
fn main() {
    let args = std::env::args();
    let _skip_executable = args.next();

    for arg in args {
        // ...
    }
}
```

</section>