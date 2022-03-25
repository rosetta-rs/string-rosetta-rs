# Rust String Parsing Benchmarks

This repo tries to assess Rust string types.

We currently compare:

Name                                                  | Heap  | Small-string | `&'static str` | Mutable | Notes
------------------------------------------------------|-------|--------------|----------------|---------|-----
`String`                                              | **Y** | N            | N              | **Y**   | Universal
`Cow<'static, str>`                                   | **Y** | N            | **Y**          | N       |
[`compact_str`](https://crates.io/crates/compact_str) | **Y** | 24 bytes     | N              | N       |
[`flexstr`](https://crates.io/crates/flexstr)         | **Y** | 22 bytes     | **Y**          | N       | O(1) clone
[`kstring`](https://crates.io/crates/kstring)         | **Y** | 15 bytes     | **Y**          | N       | Optional O(1) clone, optional 22 byte small string, Ref/Cow API for preserving `&'static str`
[`smartstring`](https://crates.io/crates/smartstring) | **Y** | 23 bytes     | N              | **Y**   |
[`smol_str`](https://crates.io/crates/smol_str)       | **Y** | 22 bytes     | N              | N       | O(1) clone, Whitespace storage optimizations

Suggestions:
- Generally, `String`
- If you deal mostly with string literals but want some flexibility (like
  [clap](https://github.com/clap-rs/clap/)), generally you'll want
  `Cow<'static, str`>
- If a profiler says your strings are a problem:
  - Try different crates and settings for that crate out with a profiler
  - O(1) clone tends to be better with large allocations
  - For short-lived programs, look into string interning

# Results

`new`:
[![`new`](runs/2022-03-25/new/report/lines.svg)](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-25/new/report/index.html)

`clone`:
[![`clone`](runs/2022-03-25/clone/report/lines.svg)](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-25/clone/report/index.html)

`access`:
[![`access`](runs/2022-03-25/access/report/lines.svg)](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-25/access/report/index.html)

# Special Thanks

- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
