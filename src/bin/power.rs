use rust_gpiozero::*;
fn main() {

    // // Use Pin 3, for power cycling, will reboot when shorted to ground
    let mut button = Button::new_with_pulldown(3);

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    button.wait_for_press(None);


    println!("Sleeping");

    let output = std::process::Command::new("shutdown")
        .arg("-h")
        .arg("now")
        .output()
        .expect("Failed to execute command");
}

