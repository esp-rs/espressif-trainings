use anyhow::{bail, Result};
use core::str;
use embedded_svc::{
    http::{client::Client, Status},
    io::Read,
};
use esp32_c3_dkc02_bsc::wifi::wifi;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::client::{Configuration, EspHttpConnection},
};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = bsc::wifi::wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    // TODO your code here
    //get(...)?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> Result<()> {
    // 1. Create a new EspHttpClient. (Check documentation)

    // 2. Open a GET request to `url`

    // 3. Submit write request and check the status code of the response. Successful http status codes are in the 200..=299 range.

    // let response = writer...;
    // let status = ...;
    // println!("response code: {}\n", status);

    // 4. If the status is OK, read response data chunk by chunk into a buffer and print it until done.
    // 5. Try converting the bytes into a Rust (UTF-8) string and print it.

    Ok(())
}
