#![no_std]

#[derive(Debug)]
pub enum PlatformError {
    InitError,
}

pub trait PlatformIo {
    fn led_on(&mut self);
    fn led_off(&mut self);

    fn sleep_ms(&mut self, delay_ms: u32);
}
