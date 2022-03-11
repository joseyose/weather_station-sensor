mod humidity_sensor;
mod post;

use post::{new_weather_entry};

fn main() {
    println!("Initializing my humidity sensor!\n");
    new_weather_entry();
    // post::test_weather_post();
}
