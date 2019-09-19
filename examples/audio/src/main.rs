use azurite;
use winapi::um::winuser;

fn main() {
    println!("Hello, world!");
    unsafe{
        let window = azurite::window_initialize();
        println!("I was here!");
        // winuser::ShowWindow(window, winuser::SW_SHOW);
    }
    while(true) {

    }
}
