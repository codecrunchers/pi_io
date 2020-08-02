//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds

use rust_gpiozero::*;

fn main() {

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let mut led = LED::new(17);
    while true {

        if network_available() {
            led.blink(1.0,4.0);
        }else{

            led.blink(1.0,1.0);
        }

        std::thread::sleep_ms(10000);
    }

}




fn network_available() -> bool {
    match std::process::Command::new("cat")
        .arg("/sys/class/net/wlan0/carrier")
        .output() {
            Ok(s) => {
                println!("{}", s.status.success());
                s.status.success()
            }
            Err(e) =>  false,
        }

}
