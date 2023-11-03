# Embasy tutorial on STM32F3 Discovery board

This repository contains my implementation of the Embassy tutorial series by [AppolloLabs.bin](https://apollolabsblog.hashnode.dev/series/rust-embassy) using the STM32F3DISCOVERY board.

## Environment setup

The environment setup for this project is the same as for the new Rust Embedded Discovery Book (whoch uses the micro:bit board). Instructions can be found [here](https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html).

## How to run

1. Build and flash:

```shell
cargo embed --bin <binary>
```

2. To debug, in a separate terminal:

```shell
./gdb.sh
```
