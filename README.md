# Rust String Parsing Benchmarks

This repo tries to assess Rust string types.

We currently compare:

Name                                                 | Heap  | Small-string | `&'static str` | Mutable | Notes
-----------------------------------------------------|-------|--------------|----------------|---------|-----
`String`                                             | **Y** | N            | N              | **Y**   | Universal
`Cow<'static, str>`                                  | **Y** | N            | **Y**          | N       |
[`compact_str`](https://crates.io/crates/kstring)    | **Y** | 24 bytes     | N              | N       |
[`flexstr`](https://crates.io/crates/kstring)        | **Y** | 22 bytes     | **Y**          | **Y**   | O(1) clone
[`kstring`](https://crates.io/crates/kstring)        | **Y** | 15 bytes     | **Y**          | N       | Optional O(1) clone, optional 22 byte small string
[`smartstring`](https://crates.io/crates/kstring)    | **Y** | 23 bytes     | N              | **Y**   | **Assumes `String` memory layout**
[`smol_str`](https://crates.io/crates/kstring)       | **Y** | 22 bytes     | N              | N       | O(1) clone, Whitespace storage optimizations

# Results


# Special Thanks

- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
