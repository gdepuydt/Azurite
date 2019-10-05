// #![windows_subsystem = "windows"]
use azurite::window;

#[cfg(windows)]
fn main() {
    println!("Hello, world!");
    let mut window = window::create_window("Azurite").unwrap();
    //let optional_functions = window::load_optional_functions();
    if let Some(func) = window::OPTIONAL_FUNCTIONS.SetProcessDpiAwareness {
        println!("Static initialization complete.");
    }
    // Test to see we load the system functions only once
    if let Some(func2) = window::OPTIONAL_FUNCTIONS.SetProcessDpiAwareness {
        println!("Static initialization complete, again.");
    }
    
    loop {
        if !window::handle_message(&mut window) {
            break;
        }
    }
}
