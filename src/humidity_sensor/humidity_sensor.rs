use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
// use serde::{Serialize, Deserialize};
// use std::{thread, time};

// use std::path::Path;
// use std::fs::{File, OpenOptions};

// use std::io::Write;

use crate::post::structs::weather::Weather;


pub fn initialize() -> BME280<I2cdev, Delay> {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();

    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    bme280.init().unwrap();

    return bme280
}

pub fn collect_measurements(mut sensor: BME280<I2cdev, Delay>) -> Weather {
    let local = chrono::offset::Local::now();
    let measurements = sensor.measure().unwrap();

    Weather {
        temperature: measurements.temperature as f64,
        humidity: measurements.humidity as f64,
        pressure: measurements.pressure as f64,
        date: local,
    }

    // loop {
    //     let measurements = sensor.measure().unwrap();
    //     println!("Relative Humidity: {:.2}%", measurements.humidity);
    //     println!("Temperature F: {:.2} degrees", measurements.temperature * 9.0 / 5.0 + 32.0);
    //     println!("Temperature C: {:.2} degrees", measurements.temperature);
    //     println!("Pressure: {} pascals", measurements.pressure);
    //     println!("----------------------------------");

    //     // write_to_file(measurements.temperature, measurements.humidity);
    //     thread::sleep(time::Duration::from_secs(5));
    // }
}

// fn write_to_file(temp: f32, humidity: f32) {
//     let path = Path::new("./test.csv");
//     if !path.exists() {
//         println!("csv does not exist yet");
//         // write!("./test.csv", "temperature, humidity").unwrap();
//     } else {
//         let mut file = OpenOptions::new()
//             .write(true)
//             .append(true)
//             .open("./test.csv")
//             .unwrap();

//         let output: String = format!("{}, {}", temp.to_string(), humidity.to_string());
//         writeln!(file, "{output}");
//     }

//     // println!("writing to file");
// }