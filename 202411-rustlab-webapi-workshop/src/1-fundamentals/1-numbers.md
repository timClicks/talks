# Numbers

<section class="slide">

## 3 families of numbers

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


- signed integers ()
- floating point
- \\\

| Fixed-width integers        | Signed | Unsigned |

| Cpu              |            |          |
| Floating point numbers              |        |          |

</section>

<section class="slide">

## Types are not compatible


This will be important when we come to work with collections.
Only `usize` values can be used to index them.


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
