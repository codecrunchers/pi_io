use rust_gpiozero::*;
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {

    let mut button = Button::new(17);
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");


    let matches = App::new("Rusty Cam")
        .version("1.0")
        .author("Alan R. <alan@alanryan.namegmail.com>")  .version("1.0")
        .about("Will Invoke WPS Script")
        .arg(Arg::with_name("script")
             .short("s")
             .long("script")
             .value_name("path")
             .help("Sets a custom Script to Execute")
             .takes_value(true))
        .get_matches();

    let config = matches.value_of("script").unwrap_or("/opt/wps/wps-config.sh");
    println!("Using {} as WPS Script", config);

    loop {
        button.wait_for_press(None);
        println!("WPS button was pressed, running {}", config);

        let output = std::process::Command::new(config)
            .output();

        match output {
            Ok(r) =>   println!("{:?} {:?}", std::str::from_utf8(&r.stdout), ""),
            Err(e) => println!("{}", e )
        };

        println!("Sleeping for a while..zzzzzz.");
        std::thread::sleep_ms(25000);
    }
}

