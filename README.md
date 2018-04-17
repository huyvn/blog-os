# A small OS written in Rust

Following the instruction from [Writing an OS in Rust](https://os.phil-opp.com/second-edition/) by Philipp Oppermann

## Prerequesites
* Install Rust (https://rustup.rs/)
* Run Linux <br>You can refer to the [blog post](https://os.phil-opp.com/freestanding-rust-binary/#overwriting-the-entry-point) for building on Windows and macOS

## Building

1. Clone this Repo
```
git clone https://github.com/huyvn/blog-os.git
```
1. Build the binary
```
cargo rustc -- -Z pre-link-arg=-nostartfiles
```

## License

The source code is licensed under Apache License (Version 2.0).
