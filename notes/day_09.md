# Day 9: Better Organization
I enjoyed this day's puzzle. It was fun building a (very) rudimentary assembly instruction interpreter.

However, I got a lot more enjoyment taking care of issues I had with the project's organization, primarily due to the binary crate having direct coupling to each day's module. Adding a new module for a new day required tedious steps:

1. Create a module for the new day.
1. Add empty `part_one()` and `part_two()` functions to the new module.
1. In `main.rs`:
    1. Add a `mod` statement to include the new module.
    1. Update the `LAST_DAY_IMPLEMENTED` constant to the number of the new day. This supports command line argument validation.
    1. Add a branch for the new day to the `match` statement in `main()`, calling the `run()` function with the new module's `part_one()` and `part_two()` functions as arguments.

To reduce this repetition, I created a `days` module and moved all of the days' modules into it as submodules. Then, in the `days` module, I created a `DayRunner` struct that runs the available solutions of a single day. Finally, I refactored everything in `days` to work with the new struct.

Now, `main.rs` no longer needs modification. It merely makes calls to the `days` API. And when I start a new day, all I have to do is:
1. Create a new submodule in `days` with this boilerplate code:
    ```rust
    use super::DayRunner;

    pub fn runner(data: Vec<String>) -> DayRunner {
        DayRunner::new(data, None, None)
    }
    ``` 
1. In the `days` module, include the new day's module and add its `runner()` function to the vector returned by the `builders()` function.

The project will compile and run. This is a nice place to start implementing the new day's solutions, which only requires modifying that day's module.

## What I Learned
Here are a few things I learned from doing all of this.

### Functions as Member Fields
Calling a function as a struct field requires either binding the field's value to a local variable, as in:
```rust
let f = self.part_one;
f(self.data);
```
or wrapping the field in parentheses, like so:
```rust
(self.part_one)(self.data);
```
Without these parentheses, the compiler complains about no such method existing. Even though the Rust compiler is often quite helpful, it doesn't offer an accurate suggestion in this case:
```
error[E0599]: no method named `part_one` found for reference `&DayRunner` in the current scope
  --> src/days/mod.rs:45:14
   |
45 |         self.part_one(self.data);
   |              ^^^^^^^^----------- help: remove the arguments
   |              |
   |              field, not a method
```
### Relative Paths
Each day's submodule can use a relative path to the new `DayRunner` struct instead of an absolute path. So instead of this:
```rust
use crate::days::DayRunner;
```
they can use this:
```rust
use super::DayRunner;
```
This isn't news to me, but its effect was nice to experience for the first time. When I changed the name of the module from `lib` to `days`, I didn't have to update the name in the submodules.
