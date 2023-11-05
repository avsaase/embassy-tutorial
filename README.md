# Embasy tutorial on STM32F3 Discovery board

This repository contains my implementation of the Embassy tutorial series by [AppolloLabs.bin](https://apollolabsblog.hashnode.dev/series/rust-embassy) using the STM32F3DISCOVERY board.

## Environment setup

The environment setup for this project is the same as for the new Rust Embedded Discovery Book (the version that uses the micro:bit board, not the old version of book that uses the board that this repo uses). Instructions can be found [here](https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html).

## How to run

1. Build and flash:

```shell
cargo embed --bin <binary>
```

2. To debug, in a separate terminal:

```shell
./gdb.sh
```

## Differences with the tutorial series

- Part 3: Instead of driving an LED, I used a simple analog servo.
- Part 4: I didn't have an analog sensor on hand so I skipped this part.
- Part 5: Instead of a BMP-180 barometer sensor, I used a LM75 temperature sensor.
