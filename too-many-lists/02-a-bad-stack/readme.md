- Some values of an Enum need less space than other. But in mem, they still
  reserve enough space to be able to become other values if needed.

- The teaching style of this book is cool. It mentions possible mem layout of
  the linked lists, explain pros and cons and then solutions to implement those
  layouts in Rust.

- The code for 2.1 has quite a few magics:
  - Relace `self.head` with `Empty`, get the old value of head as the result.
  - Use `match` with that result to check, and
  - If that result is a `More`, then replace the `self.head` once again.
