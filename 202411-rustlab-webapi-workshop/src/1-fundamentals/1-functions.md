<section class="slide">

# Functions

</section>
<section class="slide">

Functions in Rust come in three general forms.

- free functions
- methods
- static methods

Other callable

- closures

</section>

<section class="slide">

## Free functions

<section class="slide">

```rust,editable
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn increment(n: i32) -> i32 {
    n + 1
}

fn main() {
    let a = 42;
    if is_even(a) {
        println!("{}")
    }
}
```
<!-- 
<ul class="fragment">
    <li class="fragment">Free functions</li>
    <li class="fragment">Methods on types </li>
</ul> -->

</section>

<section class="slide">

## Methods

```rust,editable
"firenze".to_uppercase()
```

</section>
<section class="slide">

Methods have a special parameter, `self`.
For reasons that we'll see later, it's normally seen as `&self` and `&mut self`.
The forms indicate the intended access pattern, but the details not essential at this point.

- Read only: `&self`
- Read/write: `&mut self` and `self`

<!--

It can have

- `self` - move ownership of self into the caller.
- `&self` - read access
- `&mut self` - read/write access to the calling
- -->

</section>

<!--
<section class="slide">

Functions are related to a series of traits:

<ul class="fragment">
    <li class="fragment"><code>Fn</code></li>
    <li class="fragment">Methods on types </li>
</ul>

</section>
-->