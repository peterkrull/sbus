# Efficient SBUS parser for Rust
A `no_std` parser for the SBUS RC protocol in Rust. 

## This branch : Testing bitvec
This branch tests how using the `bitvec` library to do the bit-conversion, instead of the manual bit-shifting, would impact performance. The benchmarks included in this branch show that using bitvec is 2-3 times slower than the hard-coded bitshifting method otherwise used.