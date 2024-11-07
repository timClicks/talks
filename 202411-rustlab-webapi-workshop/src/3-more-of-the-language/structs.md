<section class="slide">

# Structs

</section>
<section class="slide">

Structs are the standard tool to build your own data types.

</section>

<section class="slide">

```rust
struct Circle {
    origin: (f32, f32),
    diameter: f32,
}

fn main() {
    let c = Circle { origin: (1.0, 0.0), diameter: 3.0 };
    match c {
        Circle { origin: (1.0..5.0, y), diameter: _ } => println!("y = {y}"),
        Circle { origin: (x, y), diameter: 0.0..1.0 }   => println!("x = {x}, y = {y}"),
        Circle { diameter, .. }        => println!("diameter = {diameter}, other fields were ignored"),
    }
}
```

</section>