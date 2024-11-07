<section class="slide">

# Text

</section>
<section class="slide">

Three main types:

- `char` - a single "character", more formally a "Unicode scalar value", encoded as UTF-32 (4 bytes)
- `&str` - a slice to a span of bytes that is UTF-8 encoded
- `String` - a heap-allocated string

<p class="fragment"> More specialised text types</p>

<ul class="fragment">
    <li><code>std::path::Path</code></li> <li><code>std::ffi::OsString</code></li> <li><code>std::ffi::CString</code></li>
</ul>

</section>
