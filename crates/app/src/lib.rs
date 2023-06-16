#![no_std]

use embedded_hal::digital::v2::OutputPin;
use platform_io::PlatformIo;

pub fn main_loop(platf: &mut dyn PlatformIo){
    platf.led_on();
    platf.sleep_ms(100);
    platf.led_off();
    platf.sleep_ms(100);
}
