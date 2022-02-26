#![no_std]
#![no_main]

use panic_probe as _;
use cortex_m_rt::entry;

use stm32l4 as _;

#[entry]
fn main() -> ! {
    // println!("Hello, world!");
    loop { }
}

