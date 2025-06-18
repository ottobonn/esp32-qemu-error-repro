#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::main;

#[main]
fn main() -> ! {
    loop {}
}