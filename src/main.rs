#![no_std]
#![no_main]


use embedded_time::rate::*;

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

use rp_pico as bsp;
use bsp::entry;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};


#[entry]
fn main() -> ! {
    info!("program start");
    let mut      pac = pac::Peripherals::take().unwrap();
    let         core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let          sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32.Hz();
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz.integer(),
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    loop {
        info!("On!");
        led.set_high().unwrap();
        delay.delay_ms(100);
        info!("Off!");
        led.set_low().unwrap();
        delay.delay_ms(100);
    }
}
