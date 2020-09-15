- I like the naming convention in Go (noun for struct and interface, verb for
  methods) more than Rust. Rust doesn't seem to have a naming convention for
  trait (`Debug`, `Display`, `Drop`). Methods and functions names in Rust are
  verb, though.

- It's hard to implement recursive data structure in Rust, such as Linked List,
  due to the safety features (i.e ownership and lifetime). Better to read [Too
  many linked lists](https://rust-unofficial.github.io/too-many-lists/) after
  this book.

- Note about [Deref coercion](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods)
  that auto convert between type that implement `Deref` trait.

- About `RefCell<T>`:

  > With RefCell<T>, if you break these rules, your program will panic and exit.
  > ...
  > The RefCell<T> type is useful when you’re sure your code follows the
  > borrowing rules but the compiler is unable to understand and guarantee that.
