<section class="slide">

# Shared references

</section>
<section class="slide">

A shared reference---also known as an immutable reference---provides a way to access another value without changing ownership of the value.

They're specified with an ampersand, e.g. `&T`.
<span class="fragment">They provide read only access to <code>T</code>. The referenced data cannot change.</span>

<span class="fragment">They are called shared references because multiple shared references are allowed to co-exist, unlike unique/mutable references.</span>

</section>
<section class="slide">

```rust,editable
fn main() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    println!("r: {}", *r);

    r = &b;
    println!("r: {}", *r);
}
```

</section>