pub mod structs;
use structs::weather::Weather;

use rand::Rng;
use std::{thread, time};

use crate::humidity_sensor::get_measurements;

#[tokio::main]
pub async fn new_weather_entry() {
    let data = get_measurements();
    println!("{:?}", data);

    let mut counter = 0;

    while counter < 200 {
        counter += 1;

        let local = chrono::offset::Local::now();

        let new_weather = Weather {
            temperature: data.temperature,
            humidity: data.humidity,
            pressure: data.pressure,
            date: local,
        };

        println!("Posting: {:#?}", new_weather);

        let _new_post = reqwest::Client::new()
            .post("http://0.0.0.0:8000/weather/new-entry")
            .json(&new_weather)
            .send()
            .await
            .expect("Unable to contact the server!");

        thread::sleep(time::Duration::from_secs(15));
    }
}   

#[tokio::main]
pub async fn test_weather_post() {
    let mut rng = rand::thread_rng();
    let mut counter = 0;

    while counter < 10 {
        counter += 1;

        let local = chrono::offset::Local::now();

        let mut temp: f64 = rng.gen();
        temp = temp * 100.0;

        let mut humi: f64 = rng.gen();
        humi = humi * 100.0;

        let mut pres: f64 = rng.gen();
        pres = pres * 10000.0;

        println!("Successfully running from here!");

        let new_weather = Weather {
            temperature: temp,
            humidity: humi,
            pressure: pres,
            date: local,
        };

        println!("Posting: {:#?}", new_weather);

        // let _new_post = reqwest::Client::new()
        //     .post("http://127.0.0.1:8080")
        //     .json(&new_weather)
        //     .send()
        //     .await
        //     .expect("Unable to contact the server!");

        thread::sleep(time::Duration::from_secs(5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_weather_measures() {
        let data = get_measurements();
        println!("{:?}", data);
    }
}
