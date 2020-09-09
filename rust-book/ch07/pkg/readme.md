Module system in Rust seems to take file name more serious than Java and Go
(both only cares about folder).

In Rust, directory is not a module, it just a place to store code for other
modules. So, to group code related to a `sub` "features", we 2 2 options:

-   Use `sub/mod.rs` that just re-exports everything under `sub`. This is from
    Rust 2015.

```
sub/
    a.rs
    b.rs
    mod.rs
```

-   Use `sub.rs` which is sibling to `sub/` that can also re-exprt things under
    `sub/`. This is from Rust 2018.

```
sub/
    a.rs
    b.rs
sub.rs
```

I prefer the 2015 style.
