# Pollard's Rho Prime Factorization in Rust

This project is a Rust implementation of Pollard's Rho prime factorization algorithm, leveraging the power of the GNU Multiple Precision Arithmetic Library (GMP). The implementation utilizes bindings provided by `gmp-mpfr-sys` and is abstracted through the `rug` library for ease of use.

## Why GMP?

I selected GMP for this implementation to achieve superior performance compared to my initial implementation of the algorithm in SageMath. During my research, I discovered that SageMath internally uses GMP for its arithmetic operations, which explained why my first attempt didn't outperform it. By directly integrating GMP in this Rust implementation, I aimed to match and potentially exceed the performance of the SageMath version. The downside to this is that compilation with gmp-mpfr-sys can be a bit tricky since it compiles GMP from it's C-source. For compilation instructions i therefore refer to [gmp-mpfr-sys documentation](https://crates.io/crates/gmp-mpfr-sys) 
