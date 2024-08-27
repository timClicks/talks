# import rust

An introduction to the confusing concepts of Rust for Python developers.

## References

- [Conf deep link](https://talks.kiwipycon.nz/kiwi-pycon-2024/talk/8WN3FK/)
- [Wayback Machine link in case of link rot](https://web.archive.org/web/20240827210530/https://talks.kiwipycon.nz/kiwi-pycon-2024/talk/8WN3FK/)

## Description

An informative guide to using the Rust programming language to support Python projects. This talk has two aims. Its first is to demystify Rust and show you a subset of the language that's far less intimidating that you may have been lead to believe. The second is to guide you through a set of decision criteria for considering supplementing your Python project with the language.
Motivation

One of Rust's most celebrated aspects is that makes it practical to create high-performance tools. They run fast, have low resource requirements and are easy to distribute.

But is that enough to justify learning a new programming language and bolt that on to your existing stack?

In the first section of the talk, we'll learn some of Rust's more confusing concepts, such as ownership, lifetimes and borrowing as well as its answer to exceptions, with code examples written in Python.

We'll then create a small extension module and show how easy it is to distribute Rust as a wheel.

The final section will be a discussion of how to decide whether learning Rust is worth it. Rust isn't the only way to write extension modules, and some alternatives are much closer to Python than Rust is.
