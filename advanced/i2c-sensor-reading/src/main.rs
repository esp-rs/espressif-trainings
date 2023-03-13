use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};
use icm42670p::{DeviceAddr, ICM42670P};
use shared_bus::BusManagerSimple;
use shtcx::{self, PowerMode};
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

// goals of this exercise:
// instantiate i2c peripheral
// implement one sensor, print sensor values
// implement second sensor on same bus to solve an ownership problem

fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // 1. Instantiate the SDA and SCL pins, correct pins are in the training material.

    // 2. Instantiate the i2c peripheral,I2cDriver, using a I2cConfig of 400kHz

    // 3. Create an instance of the SHTC3 sensor.

    // 4. Read and print the sensor's device ID.

    loop {
        // 5. This loop initiates measurements, reads values and prints humidity in % and Temperature in °C.
        FreeRtos.delay_ms(500u32);
    }
}
