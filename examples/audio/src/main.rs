// #![windows_subsystem = "windows"]
use azurite;

#[cfg(windows)]
fn main() {
    println!("Hello, world!");
    let mut window = azurite::create_window("Azurite").unwrap();
    loop {
        if !azurite::handle_message(&mut window) {
            break;
        }
    }
}
