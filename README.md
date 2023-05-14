# tiny-structures

This repository contains data structures and algorithms I have studied and explored for fun!
The data structures are implemented in Rust. Some of them use raw pointers and the tests 
should be run with [`miri`](https://github.com/rust-lang/miri)
to check for [undefinded behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html):

```bash
rustup toolchain install nightly --component miri
rustup override set nightly

cargo miri test
```
