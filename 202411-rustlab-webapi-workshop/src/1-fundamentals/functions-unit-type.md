<section class="slide">

# Unit type

</section>
<section class="slide">

## Functions without an explicit return type return `()` ("unit")

</section>

<section class="slide">

```rust, editable
// main returns (), not an integer
fn main() -> () {
    println!("error code not required");
}
```

</section>

<section class="slide">

## Expressions terminated with a semi-colon (`;`) return `()`

```rust,editable
fn main() -> () {
    let answer = {
        1 + 2
    };
}
```

</section>