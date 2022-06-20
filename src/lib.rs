#![no_std]

use defmt_rtt as _; // global logger
use nrf52833_hal as _; // memory layout

use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf() // Cancel dubble panic logging
}

// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() {
    loop {
        cortex_m::asm::bkpt();
    }
}