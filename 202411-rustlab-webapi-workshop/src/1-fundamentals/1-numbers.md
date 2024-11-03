# Numbers

<section class="slide">

## 3 families of numbers

</section>

<section class="slide">

 - Integers
  - signed integers (fixed-width)
   - unsigned integers (fixed-width)
 - Floating point numbers
 - signed integer (width depends on target architecture)
 - unsigned integer (width depends on target architecture)

<!-- | Fixed-width integers        | Signed | Unsigned |
|----------------------------
| Cpu              |            |          |
| Floating point numbers              |        |          | -->

</section>

<section class="slide">

## Signed Integers

</section>
<section class="slide">

```rust,editable
# show i8, i16, i32, i64

fn main() {
    let a = -5;
    let b = 12345;

    println!("{a} + {b} = {}", a+b);
}
```

</section>

<section class="slide">

## Types are not compatible

In mathematics, if you want to calculate half of \\(a\\),
<span class="fragment">you can either use multiplication </span><span class="fragment">(\\(a \times 0.5 \\))</span>
<span class="fragment"> or division </span><span class="fragment">(\\(a \div 2 \\)).</span>

<span class="fragment"> Rust won't allow this,
because floating point and integers are incompatible.</span>
<span class="fragment"> Even integers with different widths are incompatible.</span>

</section>
<section class="slide">

Understand Rust's strictness will be important when we come to work with collections.
<span class="fragment">Only `usize` values can be used to index them.</span>

</section>
<section class="slide">

```rust,editable
fn main() {
    let i = 1.1;
    let elements = [1, 2, 3];
    let element = elements[i];

    println!("{elements}")
}

// TODO: show - indexing working
```

</section>
<section class="slide">


```rust,editable
fn main() {
    let items = [1, 2, 3, 4, 5];
    let item = items[1.1];

    println!("{item}")
}
```


</section>


<section class="slide">

# Summary

</section>
<section class="slide">

## Principles to learn

Rust is:

- strongly typed
- precise
- explicit

## Strongly typed

Types cannot pretend to be other types.

## Explicit

Integer promotion does not exist.
A `u16` won't become a `u64` when needed.
There are very few places where *implicit control flow* is part of the language.


</section>
