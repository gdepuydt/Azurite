// #![windows_subsystem = "windows"]
use azurite::window;

#[cfg(windows)]
fn main() {
    println!("Hello, world!");
    window::init();
    println!("init finished.");
    let mut window = window::create_window("Azurite").unwrap();
    loop {
        if !window::handle_message(&mut window) {
            break;
        }
    }
}
