// #![windows_subsystem = "windows"]
use azurite;

#[cfg(windows)]
fn main() {
    println!("Hello, world!");
    let mut window = create_window("Azurite");
    loop {
        if !handle_message(&mut window) {
            break;
        }
    }
}
