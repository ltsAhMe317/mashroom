use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

use dialog::DialogBox;
use gl_tools::{
    draws::{vec_from_rad, Camera, Camera3D},
    gl_unit::{window::Window, GLcontext},
    glam::Mat4,
    glfw,
    ui::{
        layout::{BoundLayout, FloatLayout, LayoutPos, ListLayout, WindowLayout}, object::{rc_refcell, UIbutton, UItext}, Frame, KeyStream
    },
};
use mashroom::{HitMode, Object, Room, ToolBar};

#[derive(Clone, Copy)]
enum Action {
    Start,
    Edit,
}
fn main() {
    let mut window = Window::new(1920, 1080, "room editor", false);
    window.window.show();
    let mut context = GLcontext::with(&mut window);
    let mut look: (f32, f32) = (0f32, 0f32);
    let mut camera = Camera3D::new(&window);
    // camera.location = vec3(0f32, 5f32, 0f32);
    let room = Rc::new(RefCell::new(None));
    let action = Rc::new(RefCell::new(Action::Start));
    let mut start_frame = Frame::new();
    {
        let room = room.clone();
        let action = action.clone();
        start_frame.add(UIbutton {
            text: UItext {
                text_color: (1f32, 1f32, 1f32, 1f32),
                pos: (-200f32, 0f32),
                size: 25i32,
                text: rc_refcell("Choose File".to_string()),
            },
            action: Box::new(move || {
                *action.borrow_mut() = Action::Edit;
                let panel = dialog::FileSelection::new("select a file");
                let path = panel.show().unwrap().unwrap();
                *room.borrow_mut() = Some(Room::from_json(
                    json::parse(&fs::read_to_string(path).unwrap()).unwrap(),
                ))
            }),
            check_click: false,
        });
    }
    {
        let room = room.clone();
        let action = action.clone();
        start_frame.add(UIbutton {
            text: UItext {
                text_color: (1f32, 1f32, 1f32, 1f32),
                pos: (200f32, 0f32),
                size: 25i32,
                text: rc_refcell("New".to_string()),
            },
            action: Box::new(move || {
                *action.borrow_mut() = Action::Edit;
                let mut room_tmp = Room::new();
                room_tmp.object.push(Object::new("cao.glb", HitMode::Model));
                *room.borrow_mut() = Some(room_tmp);
            }),
            check_click: false,
        });
    }
    let mut edit_frame = Frame::new();
    edit_frame.add(ToolBar::new(&room));

  edit_frame.add({
        let text = UItext{ text_color: (1f32,1f32,1f32,1f32), pos: (0f32,0f32), size: 30, text:rc_refcell("frame here".to_string()) };
        let mut bound = BoundLayout{ pos: (0f32,0f32), bound: HashMap::new(), obj: text };
        bound.add_bound(LayoutPos::Round, 30f32);
        let window = WindowLayout::new((0f32,0f32), "create object", 30, bound);
        window
    });

    
      let mut camera_mode = false;
    let mut camera_mode_frame = Frame::new();
    let mut group = ListLayout::new(LayoutPos::Bottom, 20f32, (0f32, 0f32));
    let camera_x = rc_refcell(String::new());
    let camera_y = rc_refcell(String::new());
    let camera_z = rc_refcell(String::new());
    group.add(UItext {
        text_color: (1f32, 1f32, 1f32, 1f32),
        pos: (0f32, 0f32),
        size: 25i32,
        text: camera_x.clone(),
    });
    group.add(UItext {
        text_color: (1f32, 1f32, 1f32, 1f32),
        pos: (0f32, 0f32),
        size: 25i32,
        text: camera_y.clone(),
    });
    group.add(UItext {
        text_color: (1f32, 1f32, 1f32, 1f32),
        pos: (0f32, 0f32),
        size: 25i32,
        text: camera_z.clone(),
    });

    camera_mode_frame.add({
        let mut bound = BoundLayout{ pos: (0f32,0f32), bound: HashMap::new(), obj: group };
        bound.add_bound(LayoutPos::Round, 20f32);
        let window = WindowLayout::new((-300f32,0f32), "Camera", 25, bound);
        window
    });
    let mut key_stream = KeyStream::new();
    while !window.update() {
        context.draw_option(&mut window, |_, window| {
            let now_mode = *action.borrow();
            match now_mode {
                Action::Start => {
                    start_frame.draw(window,&mut key_stream);
                }
                Action::Edit => {
                    if camera_mode {
                        let look_vec = vec_from_rad(0f32, look.0.to_radians())
                            * window.delta_count.delta as f32;
                        let look_vec_yaw = vec_from_rad(0f32, (look.0 - 90f32).to_radians())
                            * window.delta_count.delta as f32;
                        let delta = window.delta_count.delta as f32;
                        if window.window.get_key(glfw::Key::W) == glfw::Action::Press {
                            camera.go_vec(look_vec, delta);
                        }
                        if window.window.get_key(glfw::Key::A) == glfw::Action::Press {
                            camera.go_vec(-look_vec_yaw, delta);
                        }
                        if window.window.get_key(glfw::Key::D) == glfw::Action::Press {
                            camera.go_vec(look_vec_yaw, delta);
                        }
                        if window.window.get_key(glfw::Key::S) == glfw::Action::Press {
                            camera.go_vec(-look_vec, delta);
                        }
                        if window.window.get_key(glfw::Key::Space) == glfw::Action::Press {
                            camera.location.y += window.delta_count.delta as f32;
                        }
                        if window.window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
                            camera.location.y -= window.delta_count.delta as f32;
                        }

                        let delta = window.delta_count.delta as f32 * 40f32;
                        if window.window.get_key(glfw::Key::Up) == glfw::Action::Press {
                            look.1 += delta;
                        }
                        if window.window.get_key(glfw::Key::Down) == glfw::Action::Press {
                            look.1 -= delta;
                        }
                        if window.window.get_key(glfw::Key::Right) == glfw::Action::Press {
                            look.0 -= delta;
                        }
                        if window.window.get_key(glfw::Key::Left) == glfw::Action::Press {
                            look.0 += delta;
                        }
                        look.1 = look.1.max(-80f32).min(80f32);
                        look.0 = look.0 % 360f32;
                        camera.look_rad(look.1.to_radians(), look.0.to_radians());
                    }
                    room.borrow().as_ref().unwrap().draw(camera.as_mat());
                    edit_frame.draw(window,&mut key_stream);
                    if camera_mode {
                        *camera_x.borrow_mut() = format!("x:{}", camera.location.x);
                        *camera_y.borrow_mut() = format!("y:{}", camera.location.y);
                        *camera_z.borrow_mut() = format!("z:{}", camera.location.z);
                        camera_mode_frame.draw(window,&mut key_stream);
                    }

                    if window.get_char('c') {
                        camera_mode = !camera_mode;
                    }
                }
            }
        });
        key_stream.rewind(); 
    }
}
