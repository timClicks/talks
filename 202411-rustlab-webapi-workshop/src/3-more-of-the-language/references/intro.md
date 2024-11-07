<section class="slide">

# References

</section>
<section class="slide">

Rust programmes use the term "borrowing" a lot.
Let's see what it means.

</section>
<section class="slide">

No `->` operator.
Rust will automatically de-reference references when calling methods on them.

</section>
<section class="slide">

References can never be null in Rust, so null checking is not necessary.

</section>
<section class="slide">

Its impossible to create a "dangling" pointer.

```rust,editable
fn invalid(x: &i32) -> &i32 {
    let y = *x +1;
    return &y;
}
```

</section>
<section class="slide">

This check also happens outside of function boundaries:

```rust
fn main() {
    let reference;
    {
        let number = 5;
        reference = &number;
    }

    println!("The number is: {}", reference);  
}
```

</section>
