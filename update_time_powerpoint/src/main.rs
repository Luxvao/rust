use rsautogui::mouse::*;

fn main() {
    println!("[*] Mouse position: {:?}", position());
    scroll(ScrollAxis::Y, -1);
}
