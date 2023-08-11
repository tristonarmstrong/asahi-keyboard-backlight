use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value = &args[1];
    let parsed = value.trim().parse::<i32>();

    if parsed.is_err() {
        panic!("Value entered ({}) is not number", value);
    }

    let parsed_value = parsed.unwrap();

    if parsed_value > 255 {
        panic!("value entered ({}) must be <= 255", value);
    }

    if parsed_value < 0 {
        panic!("value entered ({}) must be >= 0", value);
    }

    fs::write("/sys/class/leds/kbd_backlight/brightness", value)
        .expect("Something went wrong writing to file")
}
