# Functions

<section class="slide">

Functions in Rust come in three general forms.

## Free functions

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





<ul class="fragment">
    <li class="fragment">Free functions</li>
    <li class="fragment">Methods on types </li>
</ul>

</section>


<section class="slide">

Functions are related to a series of traits:

<ul class="fragment">
    <li class="fragment"><code>Fn</code></li>
    <li class="fragment">Methods on types </li>
</ul>

</section>