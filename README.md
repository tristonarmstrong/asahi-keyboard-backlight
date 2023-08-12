# Keyboard Backlight CLI
This essentially is just a CLI that you can install via `cargo` so you can quickly adjust the keyboard backlight of your mac

## Target OS
I build this for me to use in Asahi linux on the mac because i, for the life of me, couldnt find a ui control for the keyboard backlight.

## How To Use
1. Download this repo
2. Go to the directory
3. Run `cargo build`
4. Then Run `cargo install --path .`
5. Run `sudo kbd-bl <value>` anywhere in your terminal to adjust the backlight on the fly
6. Take a swig of taquila