<section class="slide">

# Helpful macros

</section>
<section class="slide">

## todo!()

Satisfies the type system, returning whichever type the scope requires,
and crashes the program if the part of the code is actually reached.

```rust,compile_fail
fn main() {
    todo!("A message is optional");
}
```

</section>

<section class="slide">

## dbg!()

Provides a detailed view of an expression:

```rust
fn main() {
    dbg!(1 + 1);
}
```

</section>
