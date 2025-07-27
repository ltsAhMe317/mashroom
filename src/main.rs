use std::sync::LazyLock;

use gl_tools::gl_unit::window::Window;

#[allow(unreachable_code)]
pub static RES:LazyLock<main_pak::Pak> = LazyLock::new(||{
    #[cfg(debug_assertions)]
    return main_pak::pak_path("./res");
    //else
    return main_pak::Pak::load("./main.pak"); 
});


fn main() {
    let mut window = Window::new(800, 600, "Mashroom GO!", false);
    window.window.show();

    while true{}
}

