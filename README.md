# Project structure

The project is split into three parts:
- driver abstraction
- examples based on the abstraction
- low level implementation
  - driver implementation
  - examples implementation

## Driver

The root crate contains the driver based on the **ST7920** IC.

The driver supports all the communication modes of the IC:
- parallel 4/8bits
- SPI MOSI/SCLK/CS

Abstractions are based on the `embedded-hal` crate, for the most part.
Other custom abstraction are present in the `hal` module

## Examples

The `examples` packge contains a library that implements a set of example
application that can be implemented on the low level package.

## Low level

For now the examples are being tested on a Espressif DevKitC board, so there
is only one low-level package: `esp32-wroom-32e` (the actual MCU).
