#![no_std]
#![no_main]

use panic_probe as _;
use rtt_target::{rtt_init_print, rprintln};
use cortex_m_rt::{entry, ExceptionFrame, exception};

use stm32l4::stm32l4r5 as pac;
use stm32l4xx_hal as hal;

use hal::prelude::*;
use hal::delay::Delay;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello world!");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb2);

    // PE2 is connected to the LED
    let mut led = gpioe.pe2.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);

    loop {
        rprintln!("Looping!");
        led.toggle();
        timer.delay_ms(1000u32);
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
