<section class="slide">

# Exercise: Check secrets

</section>
<section class="slide">

```shell
$ cargo new checksecrets
$ cd checksecrets
$ cargo add regex
$ cargo add clap --features=derive
```

</section>

<section class="slide">

```toml
{{ #include checksecrets/src/main.rs}}
```

</section>
<section class="slide">

`src/main.rs`

```rust,editable
{{ #include checksecrets/src/main.rs}}
```

</section>