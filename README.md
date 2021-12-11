# Advent of Code 2021 🎄

Solutions for the 2021 edition of [Advent of Code](https://adventofcode.com).

This year, I try to do this in Rust 🦀. I discover the language, so do not take this as
an idiomatic implementation.

What I learned:

- **Day01**
  - organization of the project
  - using the `lib.rs` to expose modules
  - referencing them in `main.rs`.
  - Read a file with `fs::read_to_string`,
  - iterate the lines with `.lines()`.
  - `parse().unwrap()` for a quick parsing of the line based on declared type in the
    function.
- **Day02**
  - I used `split_whitespace`, but `split_once(" ")` would have been better, because I
    could unpack the result directly.
  - If you iter a borrowed vector, you get a double reference for arguments. You can
    unpack them with `for &(x, y)` to access the value as (here) `&str`, otherwise you
    get `&&ref` and you cannot compare those to string literals. Another solution is
    to use `x.as_ref`.
- **Day03**
  - HashMap default value: `hashmap.get(x).unwrap_or(0)`
  - HashMap mutating with default value: `hashmap.entry(w).or_insert(0) += 1`
- **Day04**
  - Converting a `Vec` into an array with `try_into().unwrap()`.
- **Day08**
  - Regex with the `regex` crate (along with `lazy_static` to avoid duplicate
    compilation of regexes).
  - HashSet exists.
- **Day09**
  - Mutating a ref in a recursive function (say `fn func(&mut input)`) can be called
    internally with `fn(input)`, `fn(&mut input)` is not necessary.
- **Day10**
  - When building in release, integer will overflow with no warnings, `u32` was not
    giving the right answer, `u64` does...
