# stm32f4-pac-blinky

A minimal bare-metal blinky for the STM32F411 using direct Peripheral Access Crate (`stm32f4`) register access without a HAL.

## Purpose

Created as an exploratory project to:
- Get familiar with Rust Peripheral Access Crates (PAC).
- Understand how STM32 microcontrollers configure and control GPIO pins at the register level (`RCC_AHB1ENR`, `MODER`, `OTYPER`, `OSPEEDR`, `PUPDR`, and `ODR`).

## Target

- **MCU:** STM32F411CEU6 ("Black Pill")
- **LED:** Onboard LED on `PC13` (active low)

## Build

```bash
cargo build --release
```
