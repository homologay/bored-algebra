`bored-algebra` is intended to be a library for commutative algebra in rust. It's pretty early stages.  

## What's planned

The idea is to be able to compute with general constructions over modules such as quotients, products, localization, tensor products, maybe arbitrary coproducts and products if the abstractions work out (they probably won't). Also included will be some ways to produce examples like polynomial rings and group rings. 

If that stuff works out then maybe more fun things will be added like Grobner bases or algorithms to test for irreducibility. 

I am aiming to have few dependencies. I will use `thiserror` for error handling and I am experimenting with the libraries `rug`, `frunk`, and `quickcheck`, to see if they may be useful for this project. 

## Code structure

There are currently two crates:

`mod_r` is the central crate consisting of abstractions and constructions around modules over a commutative ring

`cat` is experimenting with `frunk` 

## Documentation

This library is not on `crates.io`, so here are the steps to view the documentation. First, clone
the repo:
```sh
git clone https://github.com/maxinebeckie/bored-algebra
cd bored-algebra
```
Then build the documentation with the following flags, so the LaTeX renders properly.
```sh
RUSTDOCFLAGS="--html-in-header src/katex-header.html" cargo doc --no-deps --open
```
This method is from the crate `rustdoc-katex-demo`.  

## Contributing

Pretty early stages for that, but I'd take any feedback you may have :)