<section class="slide">

# Printing

</section>

<section class="slide">

One of the superpowers of dynamic languages like Python and JavaScript is that you can just print things.
<span class="fragment">Rust lacks this.</span>
<p class="fragment">For most types, you'll need to define what they should look like as a string.
<span class="fragment">Another way to say this is that you will need to be very explicit about the behaviour that you intend.</span>
</p>

</section>

<section class="slide">

## Debug vs Display

</section>

<section class="slide">

So far, we've been using `println` regularly.
<span class="fragment">It's a macro that interprets a format string provided as its first argument.</span>

<div class="fragment">

```rust
fn main() {
    let a = 12345;
    println!("{}", a);
}
```

</div>

<span class="fragment">That format string takes a few forms, and is very powerful.</span>
<span class="fragment">But it does have limitations.</span>

</section>

<section class="slide">

Arguments to `println!` can be provided as a  u can define your own named arguments:

```rust
let a = 12345;
println!("{}", a)
```

</section>

<section class="slide">

You can define your own named arguments:

```rust
fn main() {
    let a = 12345;
    println!("{number}", number=a)
}
```

</section>

<section class="slide">

You can use arguments multiple times:

```rust
fn main() {
    let a = 12345;
    println!("{number} - {number} = 0", number = a)
}
```

</section>

<section class="slide">

To pretty-print values, use `{_:#?}`, replacing `_` with the value you're interested in.


```rust
fn main() {
    let a = ([1, 2, 3, 4, 5], 0.1, [0.2, 0.3]);
    println!("{a:#?}")
}
```

</section>

