<section class="slide">

# Working with dependencies

</section>

<section class="slide">

A project's Cargo.toml file (the "manifest") contains its list of dependencies.

</section>

<section class="slide">

```bash
$ # cargo add <dependency>[@version]
$ cargo add image
    Updating crates.io index
      Adding image * to dependencies
    ...
```

</section>

<section class="slide">

# Exercise

</section>

<section class="slide">

```rust
{{ #include examples/thumbnail/src/main.rs }}
```

</section>


