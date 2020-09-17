- Pattern matching can be used in function param.

  ```rust
  fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
  }

  fn main() {
      let point = (3, 5);
      print_coordinates(&point);
  }
  ```

- Match guard: extra if after the match expression.

  ```rust
  match num {
      Some(x) if x < 5 => println!("less than five: {}", x),
      Some(x) => println!("{}", x),
      None => (),
  }
  ```
