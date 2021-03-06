### General Notes:
Cargo crates seem to require either a `src/main.rs` or `src/lib.rs`
file. `main.rs` requires imports to be used since it compiles to a binary.
`lib.rs` does not, since it's a library.

### Primitives:
- signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
- unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
- floating point: `f32`, `f64`
- `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
- `bool` either `true` or `false`
- unit type `()`, whose only possible value is an empty tuple: ()

### Compound types:
- Arrays: `[1, 2, 3]`
- Tuples: `(1, 2, 3)`


### Unit Testing:
It seems like it makes sense to add unit testing to the file
that you are unit testing. Like so:

    fn add(a:u8,b:u8) -> u8 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(4, add(2,2));
        }
    }

Note the `#[cfg(test)]`, which means this module shouldn't 
be compiled. I'm sure it does more, but I'm still learning.
`#[test]` to indicate this is a test function that should be
run.

### Documentation Testing:
Rust also compiles and runs tests and examples put in documentation
blocks. Personally, I feel like I'll be using this for public facing
methods. If I'm working in a large team of Rust developers maybe also
put it on private facing methods so people know how methods can be reused.


### Integration Testing:
TODO: Haven't figured this out yet.
Will circle back to this once I do the other lessons.