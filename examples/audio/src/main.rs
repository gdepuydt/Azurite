// #![windows_subsystem = "windows"]
use azurite::window;

#[cfg(windows)]
fn main() {
    println!("Hello, world!");
    let mut window = window::create_window("Azurite").unwrap();
    loop {
        if !window::handle_message(&mut window) {
            break;
        }
    }
}
