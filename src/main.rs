use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("ERR: Must provide an argument");
        return;
    }
    let value = &args[1];
    let parsed = value.trim().parse::<i32>();

    if parsed.is_err() {
        println!("ERR: Value entered ({}) is not number", value);
        return;
    }

    let parsed_value = parsed.unwrap();

    if parsed_value > 255 {
        println!("ERR: value entered ({}) must be <= 255", value);
        return;
    }

    if parsed_value < 0 {
        println!("ERR: value entered ({}) must be >= 0", value);
        return;
    }

    fs::write("/sys/class/leds/kbd_backlight/brightness", value)
        .expect("Something went wrong writing to file - you may not have elevated privs aka sudo")
}
