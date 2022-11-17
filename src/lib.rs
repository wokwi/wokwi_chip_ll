/*
* Wokwi Custom Chip API Beta - Rust Implementation
*
* Copyright (C) 2022, Uri Shaked. Released under the MIT License.
*/

use std::{ffi::c_void, os::raw::c_char};

pub type PinId = i32;
pub type TimerId = u32;
pub type UARTDevId = u32;
pub type I2CDevId = u32;
pub type SPIDevId = u32;
pub type AttrId = u32;
pub type BufferId = u32;

#[repr(C)]
pub struct WatchConfig {
    pub user_data: *const c_void,
    pub edge: u32,
    pub pin_change: *const c_void,
}

#[repr(C)]
pub struct TimerConfig {
    pub user_data: *const c_void,
    pub callback: *const c_void,
}

#[repr(C)]
pub struct UARTConfig {
    pub user_data: *const c_void,
    pub rx: PinId,
    pub tx: PinId,
    pub baud_rate: u32,
    pub rx_data: *const c_void,
    pub write_done: *const c_void,
}

#[repr(C)]
pub struct I2CConfig {
    pub user_data: *const c_void,
    pub address: u32,
    pub scl: PinId,
    pub sda: PinId,
    pub connect: *const c_void,
    pub read: *const c_void,
    pub write: *const c_void,
    pub disconnect: *const c_void,
}

#[repr(C)]
pub struct SPIConfig {
    pub user_data: *const c_void,
    pub sck: PinId,
    pub mosi: PinId,
    pub miso: PinId,
    pub mode: u32,
    pub done: *const c_void,
}

/// # Safety
///
/// Just a stub to specify the Chip API version.
#[no_mangle]
pub unsafe extern "C" fn __wokwi_api_version_1() -> u32 {
    1
}

extern "C" {
    /* Pin API */
    pub fn pinInit(name: *const c_char, mode: u32) -> PinId;
    pub fn pinRead(pin: PinId) -> u32;
    pub fn pinWrite(pin: PinId, value: u32);
    pub fn pinWatch(pin: PinId, watch_config: *const WatchConfig) -> bool;
    pub fn pinWatchStop(pin: PinId);

    /* Analog API */
    pub fn pinADCRead(pin: PinId) -> f32;
    pub fn pinDACWrite(pin: PinId, value: f32);

    /* Time API */
    pub fn getSimNanos() -> f64;
    pub fn timerInit(timer_config: *const TimerConfig) -> TimerId;
    pub fn timerStart(timer: TimerId, micros: u32, repeat: bool);
    pub fn timerStartNanos(timer: TimerId, nanos: f64, repeat: bool);
    pub fn timerStop(timer: TimerId);

    /* UART API */
    pub fn uartInit(config: UARTConfig) -> UARTDevId;
    pub fn uartWrite(dev: UARTDevId, buffer: *const u8, count: u32) -> bool;

    /* I2C Device API */
    pub fn i2cInit(config: *const I2CConfig) -> I2CDevId;

    /* SPI Device API */
    pub fn spiInit(config: *const SPIConfig) -> SPIDevId;
    pub fn spiStart(dev: SPIDevId, buffer: *const u8, count: u32);
    pub fn spiStop(dev: SPIDevId);

    /* Attributes API */
    pub fn attrInit(name: *const c_char, default_value: f64) -> AttrId;
    pub fn attrRead(attr: AttrId) -> u32;
    pub fn attrReadFloat(attr: AttrId) -> f64;

    /* Framebuffer API */
    pub fn framebufferInit(width: *mut u32, height: *mut u32) -> BufferId;
    pub fn bufferRead(buffer: BufferId, offset: u32, data: *const u8, data_len: u32) -> u32;
    pub fn bufferWrite(buffer: BufferId, offset: u32, data: *const u8, data_len: u32) -> u32;

    /* Debug API */
    pub fn debugPrint(message: *const c_char);
}

pub const NO_PIN: PinId = -1;

/* Pin values */
pub const LOW: u32 = 0;
pub const HIGH: u32 = 1;

/* Pin modes */
pub const INPUT: u32 = 0;
pub const OUTPUT: u32 = 1;
pub const INPUT_PULLUP: u32 = 2;
pub const INPUT_PULLDOWN: u32 = 3;
pub const ANALOG: u32 = 4;
pub const OUTPUT_LOW: u32 = 16;
pub const OUTPUT_HIGH: u32 = 16;

/* Pin edges */
pub const RISING: u32 = 1;
pub const FALLING: u32 = 2;
pub const BOTH: u32 = 3;
