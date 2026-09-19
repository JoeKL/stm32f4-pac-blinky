#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    let peripheral = stm32f411::Peripherals::take().unwrap();
    let rcc = &peripheral.RCC;
    let gpioc = &peripheral.GPIOC;

    // Enable GPIOC clock
    rcc.ahb1enr()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 2)) });

    // Set pin 13 (2y & 2y+1) to General Purpose Output (01)
    gpioc
        .moder()
        .modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << 26)) | (1 << 26)) });

    // Set to push-pull (0)
    gpioc
        .otyper()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 13)) });

    // Set to fast speed (10)
    gpioc
        .ospeedr()
        .modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << 26)) | (1 << 27)) });

    // No pull-up, no pull-down (00)
    gpioc
        .pupdr()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << 26)) });

    loop {
        // Turn LED on
        gpioc
            .odr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 13)) });

        // Delay
        for _ in 0..500_000 {
            asm::nop();
        }

        // Turn LED off
        gpioc
            .odr()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 13)) });

        // Delay
        for _ in 0..500_000 {
            asm::nop();
        }
    }
}
