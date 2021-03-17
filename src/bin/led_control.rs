//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds
use rust_gpiozero::*;
extern crate clap;
use clap::{App, Arg};
use std::time::Duration;

static STATUS_CHECK_INTERVAL: u64 = 10000;
static BLINK_COUNT: i32 = 10;

fn main() {
    let old_net_status: bool = false;
    let mut led = LED::new(18);
    led.set_blink_count(BLINK_COUNT);

    ctrlc::set_handler(move || {
        println!("Received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let matches = App::new("Rusty Cam")
        .version("1.0")
        .author("Alan R. <alan.ryan@gmail.com>")
        .version("1.0")
        .about("Will Flash Led using different patterns for Wifi Connected/Not Connected")
        .arg(
            Arg::with_name("disconnected_flash_interval")
                .short("disconnected_flash_interval")
                .long("disconnected_flash_interval")
                .value_name("disconnected_flash_interval")
                .help("Interval between flashes when 'carrier' is disconnected")
                .default_value("4")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("connected_flash_interval")
                .short("connected_flash_interval")
                .long("connected_flash_interval")
                .value_name("connected_flash_interval")
                .help("Interval between flashes when 'carrier' is connected")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("carrier")
                .short("c")
                .long("carrier")
                .value_name("carrier")
                .help("the device to query, defaults to '/sys/class/net/wlan0/carrier'")
                .default_value("/sys/class/net/wlan0/carrier")
                .takes_value(true),
        )
        .get_matches();

    let carrier = matches.value_of("carrier").unwrap();
    println!("Using {:?} as Carrier to check", carrier);

    let disconnected_flash_interval: u32 = matches
        .value_of("disconnected_flash_interval")
        .unwrap()
        .parse()
        .unwrap();
    println!(
        "Using {:?} as Disconnected Flash Interval",
        disconnected_flash_interval
    );

    let connected_flash_interval: u32 = matches
        .value_of("connected_flash_interval")
        .unwrap()
        .parse()
        .unwrap();
    println!(
        "Using {:?} as connected flash Interval",
        connected_flash_interval
    );

    loop {
        let net_status = network_available(carrier);

        if net_status != old_net_status {
            println!("Status Changed, now {} was {}", net_status, old_net_status);
            if net_status {
                led.blink(1.0, disconnected_flash_interval as f32);
            } else {
                led.blink(1.0, connected_flash_interval as f32);
            }
        }
    }
}

/**
 * Check a linux proc file or similar for a I|0
 * returns boolean rep of value in file represented by carrier
 */
fn network_available(carrier: &str) -> bool {
    match std::process::Command::new("cat").arg(carrier).output() {
        Ok(s) => s.stdout.len() > 0 && s.stdout[0] == 49,

        Err(e) => {
            println!("Error Reading Wlan Status file: {:?}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn test_network_available() -> std::result::Result<(), std::io::Error> {
        assert_eq!(false, network_available("/dev/null"));
        assert_eq!(true, network_available("/sys/class/net/lo/carrier"));
        Ok(())
    }
}
