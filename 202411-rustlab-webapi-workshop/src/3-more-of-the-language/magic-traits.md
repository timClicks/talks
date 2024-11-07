<section class="slide">

# Magic traits

While all traits provide types with extra capabilities,
some are slightly more mysterious.

</section>

<section class="slide">

## Copy

Copy changes a type's behaviour from _move semantics_ to
_copy semantics_.

</section>

<section class="slide">

## Sized

Types that implement Sized are represented in memory using a known number of bytes.
This allows them to be allocated on the stack, rather than the heap.

Sized is automatically implemented for every type that it can be.

Sized is so common that it is an implicit trait bound on all  type parameters (generics).
There is special syntax to relax this restruction: `?Sized`

</section>
<section class="slide">

## Deref

Types can implement `std::ops::Deref<Target = U>`  to "become" another type.
In conjunction with Rust's automatic de-referencing behaviour, it can generate a form of polymorphism.
This is similar to the `AsRef<U>`, but the other way around.
`AsRef` provides a reference, but `Deref` provides a value directly.

</section>
