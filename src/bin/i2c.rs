#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};

use stm32l4::stm32l4r5 as pac;
use stm32l4xx_hal as hal;

use hal::delay::Delay;
use hal::i2c::{self, I2c};
use hal::prelude::*;

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
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb2);

    // PE2 is connected to the LED
    let mut led = gpioe
        .pe2
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // PA7 -> D11
    let mut scl3 =
        gpioa
            .pa7
            .into_alternate_open_drain::<4>(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    // PC1 -> A3
    let mut sda3 =
        gpioc
            .pc1
            .into_alternate_open_drain::<4>(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl);

    scl3.internal_pull_up(&mut gpioa.pupdr, true);
    sda3.internal_pull_up(&mut gpioc.pupdr, true);

    let mut i2c3 = I2c::i2c3(
        dp.I2C3,
        (scl3, sda3),
        i2c::Config::new(100.kHz(), clocks),
        &mut rcc.apb1r1,
    );

    let mut scl =
        gpiob
            .pb6
            .into_alternate_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    scl.internal_pull_up(&mut gpiob.pupdr, true);

    let mut sda =
        gpiob
            .pb7
            .into_alternate_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
    sda.internal_pull_up(&mut gpiob.pupdr, true);

    let mut i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        i2c::Config::new(100.kHz(), clocks),
        &mut rcc.apb1r1,
    );

    let mut timer = Delay::new(cp.SYST, clocks);
        timer.delay_ms(1000u32);

    loop {
        let addr: u8 = 0x6a;
        rprintln!("Ping {:x}", addr);
        let r1 = i2c.write(addr, &[]);
        rprintln!("Result (I2C1): {:?}", r1);
        let r2 = i2c3.write(addr, &[]);
        rprintln!("Result (I2C3): {:?}", r2);

        led.toggle();
        timer.delay_ms(1000u32);
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
