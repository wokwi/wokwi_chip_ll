/*
* Wokwi Custom Chip API Beta - Rust Implementation
*
* Copyright (C) 2022, Uri Shaked. Released under the MIT License.
*/

use std::{ffi::c_void, os::raw::c_char};

/// Opaque pin identifier, returned by `pinInit()`
pub type PinId = i32;
pub type TimerId = u32;
pub type UARTDevId = u32;
pub type I2CDevId = u32;
pub type SPIDevId = u32;
pub type AttrId = u32;
pub type BufferId = u32;

/// Configuration for [pinWatch()]
#[repr(C)]
pub struct WatchConfig {
    /// Data that will be passed in the first argument to pin_change
    pub user_data: *const c_void,
    /// What pin changes we listen for ([`RISING`], [`FALLING`] or [`BOTH`])
    pub edge: u32,
    /// Called when the pin value changes
    pub pin_change: extern "C" fn(user_data: *mut c_void, pin: PinId, value: u32),
}

#[repr(C)]
/// Defines the callback for the timer
pub struct TimerConfig {
    /// Data that will be passed in the first argument to the callback
    pub user_data: *const c_void,
    /// Called when the timer fires
    pub callback: extern "C" fn(user_data: *mut c_void),
}

#[repr(C)]
/// Defines the pins, configuration, and callbacks for the UART device
pub struct UARTConfig {
    /// Data that will be passed in the first argument of the callbacks
    pub user_data: *const c_void,
    /// The RX pin (or NO_PIN to disable RX)
    pub rx: PinId,
    /// The TX pin (or NO_PIN to disable TX)
    pub tx: PinId,
    /// The baud rate (e.g. 115200)
    pub baud_rate: u32,
    /// Called for each byte received on the RX pin
    pub rx_data: extern "C" fn(user_data: *mut c_void, byte: u8),
    /// Called when data transmission on the TX pin has finished
    pub write_done: extern "C" fn(user_data: *mut c_void),
}

#[repr(C)]
/// Defines the pins, address, and callbacks for the I2C device
pub struct I2CConfig {
    /// Data that will be passed in the first argument of the callbacks
    pub user_data: *const c_void,
    /// Listen for requests matching the given I2C address (7-bit). To listen for all requests, set to 0
    pub address: u32,
    /// The SCL pin
    pub scl: PinId,
    /// The SDA pin
    pub sda: PinId,
    /// Called when the chip is addressed on the I2C bus
    pub connect: extern "C" fn(user_data: *mut c_void, address: u32, connect: bool) -> bool,
    /// Called when the microcontroller wants to read a byte of data from your chip
    pub read: extern "C" fn(user_data: *mut c_void) -> u8,
    /// Called when the microcontroller writes a byte to your chip
    pub write: extern "C" fn(user_data: *mut c_void, data: u8) -> bool,
    /// Called when the microcontroller disconnects from your chip
    pub disconnect: extern "C" fn(user_data: *mut c_void),
}

#[repr(C)]
/// Defines the pins, mode, and callbacks for the SPI device
pub struct SPIConfig {
    /// Data that will be passed in the first argument of the done callback
    pub user_data: *const c_void,
    /// The clock pin
    pub sck: PinId,
    /// The MOSI data pin (or NO_PIN to disable MOSI)
    pub mosi: PinId,
    /// The MISO data pin (or NO_PIN to disable MISO)
    pub miso: PinId,
    /// SPI mode: 0, 1, 2, or 3 (default: 0)
    pub mode: u32,
    /// Called when an SPI transaction finishes
    pub done: extern "C" fn(user_data: *mut c_void, buffer: *mut u8, count: u32),
}

/// # Safety
///
/// Just a stub to specify the Chip API version.
#[no_mangle]
#[doc(hidden)]
pub unsafe extern "C" fn __wokwi_api_version_1() -> u32 {
    1
}

extern "C" {
    /* Pin API */

    /// Initializes the given pin, and returns a pin indentifier for use with the other pin methods.
    ///
    /// The `mode` parameters configures the initial state of the pin.
    /// The following values are available:
    /// - [`INPUT`] - configures the pin as a digital input
    /// - [`INPUT_PULLUP`] - configures the pin as a digital input, and attached a pull-up register to the pin.
    /// - [`INPUT_PULLDOWN`] - configures the pin as a digital input, and attached a pull-down register to the pin.
    /// - [`OUTPUT`] - configures the pin as a digital output
    /// - [`OUTPUT_LOW`] - configures the pin as a digital output, sets the value of the pin to LOW
    /// - [`OUTPUT_HIGH`] - configures the pin as a digital output, sets the value of the pin to HIGH
    /// - [`ANALOG`] - configures the pin as an Analog pin. See the "Analog API" section below.
    ///
    /// Note: `pinInit()` can only be called from `chip_init()`. Do not call it at a later time.
    pub fn pinInit(name: *const c_char, mode: u32) -> PinId;

    /// Configures the given `pin` as digital input or output.
    ///
    /// The valid values for `mode` are the same as [pinInit()]: [`INPUT`],
    /// [`INPUT_PULLUP`], [`INPUT_PULLDOWN`], [`OUTPUT`], [`OUTPUT_LOW`], [`OUTPUT_HIGH`], and [`ANALOG`].
    pub fn pinMode(pin: PinId, mode: u32);

    /// Reads the current digital value of the pin, returns either `LOW` or `HIGH`.
    pub fn pinRead(pin: PinId) -> u32;

    /// Set the output value for a digital pin. Use the `LOW` and `HIGH` constants for `value`.
    pub fn pinWrite(pin: PinId, value: u32);

    /// Listens for changes in the digital value of the given pin.
    ///
    /// You can only have one watch for a pin at any given time.
    /// The function returns true if the watch was successfully set,
    /// or false in case there is already a watch defined for this pin
    /// (and thus the new watch was not set).
    pub fn pinWatch(pin: PinId, watch_config: *const WatchConfig) -> bool;

    /// Stops watching for changes on the given pin.
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
    pub fn uartInit(config: *const UARTConfig) -> UARTDevId;
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
pub const OUTPUT_HIGH: u32 = 17;

/* Pin edges */
pub const RISING: u32 = 1;
pub const FALLING: u32 = 2;
pub const BOTH: u32 = 3;
