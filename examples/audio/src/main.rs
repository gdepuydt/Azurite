// #![windows_subsystem = "windows"]
use ::window::window;

#[cfg(windows)]
fn main() {
    println!("Hello, world!");
    let mut window = window::create_window("Azurite").unwrap();
    //let optional_functions = window::load_optional_functions();
    if let Some(func) = window::OPTIONAL_FUNCTIONS.SetProcessDpiAwareness {
        println!("Static initialization complete.");
    }
    
    loop {
        if !window::handle_message(&mut window) {
            break;
        }
    }
}
