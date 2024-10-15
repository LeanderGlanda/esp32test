use embassy_executor::Spawner;
use log::*;
use std::{thread, time};

#[embassy_executor::task]
async fn run() {
    loop {
        info!("tick");
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    spawner.spawn(run()).unwrap();
}