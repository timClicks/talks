# Collections

<section class="slide">

- lists
- sets
- maps (dictionaries)

--

tips for working with them

- Debug printing
- Cloning

</section><section class="slide">

## Lists

</section><section class="slide">

```rust,editable
fn main() {

}

// vec![]
// Vec::new()
// Vec::with_capacity()

// - info: must be same type , e.g. Vec<T>
// - show: how to add an item
// - show: how to remove an item
// - show: how to select an item by index
// - show: how to select an item with get (mention Option<T>)
// - show: how to sort
```

</section>

<section class="slide">


```rust,editable
fn recaman(n: usize) -> i32 {
    let mut sequence = vec![0];
    let mut used = std::collections::HashSet::new();
    used.insert(0);
    
    for i in 1..=n {
        let prev = sequence[i-1];
        let next = prev - i as i32;
        
        if next > 0 && !used.contains(&next) {
            sequence.push(next);
            used.insert(next);
        } else {
            let next = prev + i as i32;
            sequence.push(next);
            used.insert(next);
        }
    }
    sequence[n]
}
```
</section>
<section class="slide">

## Exercise: Recamán Sequence

</section>
<section class="slide">

```rust,editable
fn recaman(n: usize) -> i32 {
    let mut used = std::collections::HashSet::new();
    used.insert(0);
    let mut prev = 0;

    for i in 1..=n {
        let next = prev - i as i32;

        if next > 0 && !used.contains(&next) {
            prev = next;
        } else {
            prev = prev + i as i32;
        }
        used.insert(prev);
    }
    prev
}
#
# fn main() {
#    println!("{}", recaman(10)");
# }
```

</section>