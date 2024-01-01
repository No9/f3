# f3

cargo generate template for the STM32F3DISCOVERY (the "F3")

Based of the Rust Embedded Cook Book with library bumps and hardware configuration defined.

![STM32F3DISCOVERY board](https://docs.rust-embedded.org/book/assets/f3.jpg)

## usage

For configuration and required deps see the book [installation chapter](https://docs.rust-embedded.org/book/intro/install/linux.html)

```
cargo generate https://github.com/No9/f3

cargo build
```

In a seperate terminal in the root of this project run:

```
openocd
```

In the first terminal run

```
cargo run
(gdb) continue
(gdb) stepi
```

