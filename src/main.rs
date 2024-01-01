#![no_main]
#![no_std]

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use core::fmt::Write;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hio::{self, HostStream};
use cortex_m_semihosting::hprint;
use stm32f3::stm32f303::{interrupt, Interrupt, NVIC};

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
    //let mut nvic = p.NVIC;
    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
    }
    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
    syst.set_reload(8_000_000); // 1s
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    loop {
        while !syst.has_wrapped() {}

        // trigger the `EXTI0` interrupt
        NVIC::pend(Interrupt::EXTI0);
    }
}

#[interrupt]
fn EXTI0() {
    hprint!(".");
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}

#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;
    static mut STDOUT: Option<HostStream> = None;

    *COUNT += 1;

    // Lazy initialization
    if STDOUT.is_none() {
        *STDOUT = hio::hstdout().ok();
    }

    if let Some(hstdout) = STDOUT.as_mut() {
        write!(hstdout, "{}", *COUNT).ok();
    }

    if *COUNT == 9 {
        // This will terminate the QEMU process
        panic!("");
    }
}
