<section class="slide">

# String slice (`&str`)

</section>

<section class="slide">

For C++ programmers: you think of `&str` as `std::string_view`<span class="fragment">, but the one that always points to a valid string in memory.</span>

</section>

<section class="slide">

The `format!()` macro is a convenient way to generate an owned string from dynamic values. It accepts the same format specification as `println!()`.

```rust,editable
fn main() {


}
```

</section>

<section class="slide">

## Raw string syntax

```rust
fn main() {
    println!(r#"<a href="link.html">Text</a>"#);
    println!("<a href=\"link.html\">Text</a>");
}
```

</section>