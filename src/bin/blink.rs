#![no_std]
#![no_main]

use rust_emb as _; // global logger + panicking-behavior + memory layout

use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use microbit::hal::timer::Timer;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = microbit::Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        timer.delay_ms(1000_u16);

        row1.set_high().unwrap();
        timer.delay_ms(1000_u16);
    }
}