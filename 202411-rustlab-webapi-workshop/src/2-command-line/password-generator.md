<section class="slide">

# Exercise: Password generator

</section>

<section class="slide">

About the [`fastrand` dependency](https://crates.io/crates/fastrand).

```toml
{{ #include examples/genpass/Cargo.toml::7 }}
```

</section>

<section class="slide">

main.rs

```rust,editable
{{ #include examples/genpass/src/main.rs }}
```

</section>