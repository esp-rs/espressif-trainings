use anyhow::{bail, Result};
use core::str;
use embedded_svc::{
    http::{client::Client, Method},
    io::Read,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::Peripherals,
    http::client::{Configuration, EspHttpConnection},
};
use wifi::wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    get("https://espressif.com/")?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> Result<()> {
    // 1. Create a new EspHttpClient. (Check documentation)
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    // 2. Open a GET request to `url`
    let headers = [("accept", "text/plain")];
    let request = client.request(Method::Get, url.as_ref(), &headers)?;

    // 3. Submit write request and check the status code of the response.
    // Successful http status codes are in the 200..=299 range.
    let response = request.submit()?;
    let status = response.status();

    println!("Response code: {}\n", status);

    match status {
        200..=299 => {
            // 4. if the status is OK, read response data chunk by chunk into a buffer and print it until done
            //
            // NB. see http_client.rs for an explanation of the offset mechanism for handling chunks that are
            // split in the middle of valid UTF-8 sequences. This case is encountered a lot with the given
            // example URL.
            let mut buf = [0_u8; 256];
            let mut offset = 0;
            let mut total = 0;
            let mut reader = response;
            loop {
                if let Ok(size) = Read::read(&mut reader, &mut buf[offset..]) {
                    if size == 0 {
                        break;
                    }
                    total += size;
                    // 5. try converting the bytes into a Rust (UTF-8) string and print it
                    let size_plus_offset = size + offset;
                    match str::from_utf8(&buf[..size_plus_offset]) {
                        Ok(text) => {
                            print!("{}", text);
                            offset = 0;
                        }
                        Err(error) => {
                            let valid_up_to = error.valid_up_to();
                            unsafe {
                                print!("{}", str::from_utf8_unchecked(&buf[..valid_up_to]));
                            }
                            buf.copy_within(valid_up_to.., 0);
                            offset = size_plus_offset - valid_up_to;
                        }
                    }
                }
            }
            println!("Total: {} bytes", total);
        }
        _ => bail!("Unexpected response code: {}", status),
    }

    Ok(())
}
