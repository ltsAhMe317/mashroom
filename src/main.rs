#![cfg_attr(all(target_os="windows",not(debug_assertions)),windows_subsystem = "windows")]
use std::sync::LazyLock;

use gl_tools::{ gl_unit::{window::Window, GLcontext}};
use mashroom::RES;




fn main() {
    let mut window = Window::new(800, 600, &format!("Mashroom GO! {}",include_str!("../VERSION")), false);
    let mut context = GLcontext::with(&mut window);
    window.window.show();
    while !window.update(){
        context.draw_option(&mut window, |_,window|{

        });
    }
}

