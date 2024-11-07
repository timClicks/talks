<section class="slide">

# Panics

</section>
<section class="slide">

When we're starting with Rust, we handle errors by crashing the program.
Rust calls this a _panic_.

</section>

<section class="slide">

If we want to provoke a panic ourselves, there are a few ways to do that.

</section>

<section class="slide">

Use `panic!` or `assert!`:

```rust,editable
fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 && b == 0 {
        panic!("`a` or `b` must be non-zero");
    }

    assert!(a >= 0);
    assert!(b >= 0);

    if a > b {
        gcd(b, a)
    } else if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[test]
fn known_divisors() {
    let known = [
        ((24140, 40902), 34),
        ((1989, 867), 51),
        ((75, 50), 25),
        ((48, 18), 6),
    ];

    for ((a, b), divisor) in known {
        assert_eq!(gcd(a, b), divisor);
        assert_eq!(gcd(b, a), divisor);
    }
}

fn main() {
    let a = 100;
    if < 0 {
        panic!("non-negative numbers are not allowed")
    }
}
```

</section>

<section class="slide">

Use `todo!`:

```rust,editable
fn is_prime(n: i32) -> bool {
    todo!()
}

fn main() {
    let a = 100;
    if is_prime {
        println!("Indivisible");
    }
}
```

</section>

<section class="slide">


<ul>
    <li class="fragment"><code>panic!</code><span class="fragment"> <br/>Panic immediately with an optional error message<br></span> </li>
    <li class="fragment"><code>assert!</code>, <code>assert_eq!</code>, <code>assert_ne!</code><span class="fragment"> <br/>Check to see whether a condition is satisfied and panic if it is not <br></span> </li>
    <li class="fragment"><code>todo!</code><span class="fragment"> <br/>Panic with a message that the code is not implemented<br></span> </li>
</ul>

<p class="fragment">Although the outcome is the same, each of these are semantically different.</p>

</section>

<section class="slide">

## Unwrap

</section>
<section class="slide">

We haven't seen much of the `Result` and `Option` types yet.
That's coming soon.
For now, you can think of them as being containers for some value.

They provide an `.unwrap()` method. This method will `panic`

</section>
<section class="slide">

## Expect

</section>

Expect is a more s