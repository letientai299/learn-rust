# Trait

`use rand::Rng` includes a "trait", which defines methods that random number
generators implement, and **this trait must be in scope for us to use those
methods**.

Seem like import for side-effect in Go.

This is avoid collisions in case many methods have same signature but implement
different traits.
