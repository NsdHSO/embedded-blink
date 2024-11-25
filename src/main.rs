use esp_idf_hal::adc::attenuation::DB_2_5;
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let peripherals = Peripherals::take()?;
    let adc = AdcDriver::new(peripherals.adc1)?;

    let config = AdcChannelConfig {
        attenuation: DB_2_5,
        calibration: true,
        ..Default::default()
    };

    let mut adc_pin = AdcChannelDriver::new(&adc, peripherals.pins.gpio36, &config)?;
    let mut pin_led =
        PinDriver::output(peripherals.pins.gpio15).expect("Error: Unable to set pin(r) gpio15");
    loop {
        let raw_value = adc_pin.read_raw().unwrap_or(0);
        println!("Raw value: {}", raw_value);
        let moisture_percentage = setup_calibration(raw_value);
        // Keep within 0-100%
        println!("Soil Moisture: {:.2}%", moisture_percentage);

        // Control LED based on moisture
        if moisture_percentage > 20.0 {
            pin_led.set_high().expect("Error: Unable to toggle pin");
        } else {
            pin_led.set_low().expect("Error: Unable to toggle pin");
        }

        thread::sleep(Duration::from_millis(1000));
    }
}

fn setup_calibration(raw_value: u16) -> f32 {
    // Voltage calculation (for 3.3V reference)
    let voltage = (raw_value as f32) * (3.3 / 4095.0);
    println!("Voltage: {:.2} V", voltage);

    // Soil moisture calibration
    let dry_value = 3500.0;  // Replace with your dry soil reading
    let wet_value = 800.0;   // Replace with your wet soil reading
    let moisture_percentage = ((dry_value - raw_value as f32) / (dry_value - wet_value)) * 100.0;
    let moisture_percentage = moisture_percentage.clamp(0.0, 100.0); // Keep within 0-100%
    moisture_percentage
}
