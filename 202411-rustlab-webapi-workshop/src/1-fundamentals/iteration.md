<section class="slide">

# Iteration

</section>

<section class="slide">

Iteration works with the following concepts from the language:

- `std::iter::Iter`
- `std::iter::IterMut`
- `std::iter::IntoIter`
- ownership

</section>

<section class="slide">

## For loops

</section>

<section class="slide">

For loops are the workhorse looping construct.
The `for... in ...` syntax is shorthand for calling `.into_iter()`.
This can have surprising effects relating to move semantics.

```rust,editable
fn main() {
    let items = vec![1, 2, 3, 4, 5];

    for item in items {
        println!("{item}");
    }

    // for item in items {
    //     println!("{item}");
    // }
}

// TODO: .iter() vs into_iter() vs .iter_mut()
```

</section>
<section class="slide">

## Ranges

</section>
<section class="slide">

Ranges are created with `x..y` or `x..=y`,
with the latter being an inclusive range.
`x` or `y` can be omitted to use the type's
 minimum or maximum value intead.

```rust,editable
fn main() {
    for n in 0..10 {
        println!("{item}");
    }
}

// TODO: show - difference between `0..10` and `0..=10`
// TODO: show - "infinite" ranges (`0..`)
```

</section>

<section class="slide">

## While loops

</section>

<section class="slide">

While loops continue as long as an expression evaluates to `true`.
Rust is more strict than other languages, because the expression must return a `bool` type
rather than something which is "truthy".

```rust,editable
fn main() {
    let mut counter = 0;

    while counter < 5 {
        println!("{counter}");
        counter += 1;
    }
}
```

</section>

<section class="slide">

## Higher-order functions

</section>
<section class="slide">

Rust uses higher-order programming extensively.
All iterators---types which implement the `Iterator` trait---have access
to [several methods][] which perform operations on the contents.

```rust,editable
fn main() {
    let items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let squared = items.into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .enumerate();

    for (i, item) in squared {
        println!("{i}: {item}");
    }
}
```

[several methods]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

</section>
<section class="slide">

## Important method: collect

`.collect()` pulls items an iterator into a collection.
It takes a type parameter via the so-called "turbofish" syntax `::<>`.

```rust,editable
fn main() {
    let items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let squared = items.into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .enumerate()
        .collect(); // TODO: fix compile error

    for (i, item) in squared {
        println!("{i}: {item}");
    }
}
```

</section>