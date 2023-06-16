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
    gpio,
};

use platform_data::*;

use app;

struct Platform {
    led: gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>,
    delay: cortex_m::delay::Delay,
}

impl Platform {
    fn init() -> Result<Self, PlatformError> {
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

        let pins = bsp::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let mut led_pin = pins.led.into_push_pull_output();
        let delay       = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

        Ok(Self {
            led: led_pin,
            delay: delay,
        })
    }
}

impl PlatformIo for Platform {
    fn led_on(&mut self) {
        self.led.set_high().unwrap();
    }

    fn led_off(&mut self) {
        self.led.set_low().unwrap();
    }

    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay.delay_ms(delay_ms);
    }
}

#[entry]
fn main() -> ! {
    let mut platform = Platform::init().unwrap();

    loop {
        app::main_loop(&mut platform);
    }
}
