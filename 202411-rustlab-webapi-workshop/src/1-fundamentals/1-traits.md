<section class="slide">

# Traits

</section>
<section class="slide">

Traits are essential to almost all behaviour of a data type in Rust.
<span class="fragment">They are similar to abstract base classes and interfaces in object-oriented languages.</span>
<span class="fragment">Unlike object-oriented languages, Rust's traits do not create an inheritance hierarchy, although traits can require that others are also implemented.</span>  


</section>
<section class="slide">

## Examples


<div class="fragment">

<ul class="fragment">
    <li class="fragment">Using <code>{}</code> placeholders within <code>println!</code> makes use of the <code>Display</code> trait</li>
    <li class="fragment">All operators are backed by traits, such as <code>std::ops::Add</code> for the addition operator (<code>+</code>)</li>
    <li class="fragment">Destruction is backed by the <code>Drop</code> trait</li>
</ul>

</div>

</section>
