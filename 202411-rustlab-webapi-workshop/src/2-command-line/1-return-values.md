<section class="slide">

# Error codes

</section>

<section class="slide">

A `main()` function in a Rust program does not return an integer.
<span class="fragment">Instead, it returns unit (`()`) by default.</span>

<div class="fragment">

```rust,editable
fn main() /* -> () */ {
   // ...
}
```

</div>

</section>
<section class="slide">

To return a specific error code, the easiest way is to use the `exit` function from the `std::process` module:

<div class="fragment">

```rust,editable
fn main() {
   std::process::exit(1);
}
```

</div>


</section>
<section class="slide">

Main can also return the `Result` type.
<span class="fragment"><code>Result</code> either contains an <code>Ok</code> value indicating success, or an <code>Err</code> value which indicates failure.</span>

<div class="fragment">

```rust
fn main() -> std::io::Result<()> {
   // ...
}
```

</div>

</section>