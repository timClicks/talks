<section class="slide">

# Exclusive references

</section>
<section class="slide">

Exclusive references, also known as _mutable references_ or _unique refereces_,
allow changing the value they refer to.
They look like `&mut T`, where `T` is the type of the referent.

```rust
fn main() {
    let mut point = (1, 2);
    let x = &mut point.0;
    *x = 20;
    println!("point: {point:?}");
}

// TODO: change &mut point.0 to &point.0
// TODO: show changing point.0 while x valid
```

</section>