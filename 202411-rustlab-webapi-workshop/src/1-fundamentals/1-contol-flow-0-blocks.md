# Blocks


<section class="slide">

All expressions return value.
<span class="fragment"> And every block is an expression, </span>
<span class="fragment">including conditionals.</span>

</section>

<section class="slide">

<pre><code class="language-rust edition2021 hljs">fn main() {
    <span class="fragment">let z = 13;</span>
    <span class="fragment">let x = {
        <span class="fragment">let y = 10;
        println!("y: {y}");
        z - y</span>
    };</span>
    <span class="fragment">println!("x: {x}");</span>
}
</code></pre>

</section>

<section class="slide">

```rust,editable
fn main() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}
```
</section>

<section class="slide">

Let's now look at a more complicated example.

Some questions to ask:

<ul>
    <li class="fragment">Why does <code>_midpoint</code> not include a <code>return</code> keyword?</li>
    <li class="fragment">Can we removed the <code>return</code> keyword from <code>midpoint</code> be simplified?</li>
</ul>

</section>

<section class="slide">

```rust,editable
fn _midpoint(lower: i32, upper: i32) -> i32 {
    assert!(lower < upper);
    let diff = upper - lower;
    let half_diff = diff / 2;
    lower + diff
}

fn midpoint(a: i32, b: i32) -> i32 {
    if a == b {
        return a;
    }

    if a < b {
        _midpoint(a, b)
    } else {
        _midpoint(b, a)
    }
}

fn main() {
    println!("{}", midpoint(0, 1000));
}
```

</section>
