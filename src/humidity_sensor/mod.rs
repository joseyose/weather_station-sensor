pub mod humidity_sensor;
use crate::post::structs::weather::Weather;

pub fn get_measurements() -> Weather {
    let sensor = humidity_sensor::initialize();
    humidity_sensor::collect_measurements(sensor)
}