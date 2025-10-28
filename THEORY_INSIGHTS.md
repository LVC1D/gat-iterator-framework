# Solidifier questions

## Why GATs Were Necessary

If we were to stick ewith a regular generic trait where the lifetime is introduced at the trait declaration, its methods, input and output for a given implementor would be bound only to that trait and the implementor that implements this trait and uses this lifetime.

GAT's introduce the lifetime at the trait's associated type -> this removes the tight boundaries in terms of lifetime to a certain implementor -> in fact, it's more like "borrowing the lifetime" -> the trait method returns the type with its own lifetime, and returns the lfietim back to the imlpementor.

With a fixed lifetime, calling next() twice creates a conflict: 
the first returned slice (&'a [T]) is an immutable reference that's 
still alive when we try to call next() again, which requires &mut self. 
Rust's aliasing XOR mutability rule prevents having an immutable 
reference and mutable reference to the same data simultaneously.

## The where Self: 'a Bound

Removing the `where` clause removes the clarification for the Rust compiler to be informed what this lifetime would be even related to. I don't think there would be a compiler error right away. But the code that attempts a trait method call that assumes the GAT funcitonality would fail and throw an error that a lifetime still needs to be specified.

**Correction**

The removal of the `where` clause at the type definition - as checked via `cargo check` command yields the following compiler error (self-explanatory):
```zsh
error: missing required bound on `Item`
 --> phase1/gat_iterator_framework/src/lib.rs:8:5
  |
8 |     type Item<'a>;
  |     ^^^^^^^^^^^^^-
  |                  |
  |                  help: add the required where clause: `where Self: 'a`
  |
  = note: this bound is currently required to ensure that impls have maximum flexibility
  = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87
479> for more information
```

## Standard Iterator vs Lending Iterator

You technically could write the generic `struct WindowIterator<'a, T>` and have a trait with a normal associated type, and it would even allow the associated type to be of type `&'a T`. However, this would not allow the call of next() on the instance of the iterator type more than once - because lifetime is bound to the instance created, not the returned associated type - flexibility in the API is absent.

The borrow checker prevents this because each returned slice would 
need to coexist with subsequent &mut self borrows in next() calls.
