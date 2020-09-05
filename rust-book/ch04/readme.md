# Understanding ownership

The concept of "move" instead of "shallow" copy.

```rs
let s1 = String::new("hello");
let s2 = s1;
// can't use s1 at this point since its value is "moved" to s2.
```

This language design is for avoid double-free while supporting
[Resource Acquisition Is Initialization (RAII)](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)
like C++.

The following code won't compile:

```rs
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s); // compiler error: value borrowed after move
}

fn takes_ownership(s: String) {
    println!("{}", s);
}
```

Coming from Go and Java, this is a big surprise to me.

Rust has "references" (`&`) to overcome above limitation. Resources is
immutable by default, need `mut` for mutable: `&mut`
