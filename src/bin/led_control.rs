//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds
use rust_gpiozero::*;
extern crate clap;
use clap::{App, Arg};
use std::time::Duration;

static STATUS_CHECK_INTERVAL: u64 = 10000;

fn main() {
    let mut led = LED::new(18);

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
        if network_available(carrier) {
            led.blink(
                connected_flash_interval as f32,
                disconnected_flash_interval as f32,
            );
        } else {
            led.blink(
                disconnected_flash_interval as f32,
                connected_flash_interval as f32,
            );
        }
        std::thread::sleep(Duration::from_millis(STATUS_CHECK_INTERVAL));
    }
}

/**
 * Check a linux proc file or similar for a I|0
 * returns boolean rep of value in file represented by carrier
 */
fn network_available(carrier: &str) -> bool {
    match std::process::Command::new("cat").arg(carrier).output() {
        Ok(s) => {
            let val = String::from_utf8(s.stdout).unwrap();
            println!("File Value {}", val);
            val == "1"
        }

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
        let file = std::fs::write("/tmp/foo", "1").expect("Unable to write file");
        let mut contents = String::new();
        let mut carrier_file = std::fs::File::open("/tmp/foo")?;
        carrier_file.read_to_string(&mut contents);
        println!("Contents {}", contents.clone());
        assert_eq!(true, network_available("/tmp/foo"));
        Ok(())
    }
}
