# Advent of Code 2021 🎄

Solutions for the 2021 edition of [Advent of Code](https://adventofcode.com).

This year, I try to do this in Rust 🦀. I discover the language, so do not take this as
an idiomatic implementation.

What I learned:

- **Day01**: organization of the project, using the `lib.rs` to expose modules,
  referencing them in `main.rs`. Read a file with `fs::read_to_string`, and iterate the
  lines with `.lines()`. `parse().unwrap()` for a quick parsing of the line based on the
  declared type in the function.
- **Day02**: I used `split_whitespace`, but `split_once(" ")` would have been better,
  because I could unpack the result directly. If you iter a borrowed vector, you get a
  double reference for arguments. You can unpack them with `for &(x, y)` to access the
  value as (here) `&str`, otherwise you get `&&ref` and you cannot compare those to
  string literals. Another solution is to use `x.as_ref`.
