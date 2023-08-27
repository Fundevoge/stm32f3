#![no_main]
#![no_std]

use panic_halt as _;
use stm32f3xx_hal::{delay::Delay, pac, prelude::*, rcc::RccExt, spi::Spi};

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Hello World!").ok();

    let p = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    loop {}
}
