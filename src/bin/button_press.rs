use rust_gpiozero::*;
fn main() {

    // // Create a button which is attached to Pin 17
    //let mut button = Button::new_with_pulldown(17);

    let mut button = Button::new(17);

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    while  true {
        button.wait_for_press(None);
        println!("time button was pressed");

        std::thread::sleep_ms(1000);
    }
}

