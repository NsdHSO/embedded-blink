use std::thread;
use std::time::Duration;
use esp_idf_hal::adc::attenuation::DB_11;
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   

    let peripherals = Peripherals::take()?;
    let adc = AdcDriver::new(peripherals.adc1)?;

    let config = AdcChannelConfig {
        attenuation: DB_11,
        calibration: false,
        ..Default::default()
    };
    let mut adc_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio36, &config)?;

    loop {
        // Read ADC value (raw)
        let raw_value = adc_pin.read().unwrap();

        // Convert raw ADC value to voltage (assuming 0-3.3V range and 12-bit resolution)
        let voltage = (raw_value as f32) * (3.3 / 4095.0);

        // Print the voltage
        println!("Voltage: {:?} V", voltage);
        thread::sleep(Duration::from_millis(1000));

    }
}
