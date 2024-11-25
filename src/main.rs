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

        // Convert raw ADC value to voltage
        let voltage = (raw_value as f32) * (5.0 / 4095.0);
        println!("Voltage: {:.2} V", voltage);

        // Interpret soil moisture level (0% dry, 100% wet)
        let moisture_percentage = ((4095 - raw_value) as f32 / 4095.0) * 100.0;
        println!("Soil Moisture: {:.2}%", moisture_percentage);

        if moisture_percentage > 20.0 {
            pin_led.set_high().expect("Error: Unable to toggle pin");
        }else{
            pin_led.set_low().expect("Error: Unable to toggle pin");
        }
        // Delay for readability
        
        thread::sleep(Duration::from_millis(1000));
    }
}
