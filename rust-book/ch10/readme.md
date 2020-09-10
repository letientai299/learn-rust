- Rust supports adding method on concrete type of a generic struct. I don't
  remember Java has similar mechanism.

```rust
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
  (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

- Rust has "Monomorphization", which means generating specialized versions of
  generic functions. That makes Rust generic fast, but add up compilation
  time.

- Good read on [Generic comparision between Rust and
  Java](https://gist.github.com/Kimundi/8391398). Also from there I know why
  Java Generic is called "type erasure".

- Can `impl` a trait on a type only if at least 1 of them is local to the
  crate. This is due to ["orphan rule"](https://github.com/Ixrec/rust-orphan-rules)
  (the parent is not present).
