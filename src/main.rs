use esp_idf_hal::adc::attenuation::NONE;
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let peripherals = Peripherals::take()?;
    let adc = AdcDriver::new(peripherals.adc1)?;

    let config = AdcChannelConfig {
        attenuation: NONE,
        calibration: false,
        ..Default::default()
    };
    let mut adc_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio36, &config)?;

    loop {
        // Read ADC value (raw)
        let raw_value = adc_pin.read_raw().unwrap_or(0);
        println!("Raw value: {}", raw_value);

        // Convert raw ADC value to voltage
        let voltage = (raw_value as f32) * (3.3 / 4095.0);
        println!("Voltage: {:.2} V", voltage);

        // Interpret soil moisture level (0% dry, 100% wet)
        let moisture_percentage = ((4095 - raw_value) as f32 / 4095.0) * 100.0;
        println!("Soil Moisture: {:.2}%", moisture_percentage);

        // Delay for readability
        thread::sleep(Duration::from_millis(1000))
        ;
    }
}

