use std::{cell::RefCell, collections::HashMap, fs, rc::Rc, sync::{Arc, Mutex}};

use dialog::DialogBox;
use gl_tools::ui::{
    self, layout::{BoundLayout, FloatLayout, LayoutPos, ListLayout}, object::{rc_refcell, UIbutton, UItext}, KeyStream, UIrender
};
use json::JsonValue;

use crate::Room;

pub struct ToolBar {
    group: FloatLayout<BoundLayout<ListLayout>>,
}
impl ToolBar {
    pub fn new(room: &Rc<RefCell<Option<Room>>>) -> Self {
        let mut list = ListLayout::new(LayoutPos::Right, 50f32, (0f32, 0f32));
        {
            let room = room.clone();
            list.add(UIbutton {
                check_click: false,
                text: UItext {
                    text_color: (1f32, 1f32, 1f32, 1f32),
                    pos: (0f32, 0f32),
                    size: 25i32,
                    text: rc_refcell("Save".to_string()),
                },
                action: Box::new(move|| {
                    let file =dialog::FileSelection::new("save").mode(dialog::FileSelectionMode::Save).show().unwrap().unwrap();
                    fs::write(file, format!("{}",room.borrow().as_ref().unwrap().as_json())).unwrap();
                }),
            });
        }
        {
            list.add(UIbutton {
                check_click: false,
                text: UItext {
                    text_color: (1f32, 1f32, 1f32, 1f32),
                    pos: (0f32, 0f32),
                    size: 25i32,
                    text: rc_refcell("Load".to_string()),
                },
                action: Box::new(|| {}),
            });
        }
        let mut list = BoundLayout{ pos: (0f32,0f32), bound: HashMap::new(), obj: list };
        list.add_bound(LayoutPos::Left, 30f32);
        list.add_bound(LayoutPos::Top, 30f32);

        let float = FloatLayout::new(list, (LayoutPos::Left, LayoutPos::Top));
        

        Self { group: float }
    }
}
impl UIrender for ToolBar {
    fn draw(&self) -> Option<&gl_tools::gl_unit::FrameBuffer> {
        None
    }
    fn update(&mut self,window: &mut gl_tools::gl_unit::window::Window,key_stream:&mut KeyStream) {
        self.group.update(window, key_stream);
    }
    fn fast_draw(&self, window: &mut gl_tools::gl_unit::window::Window) {
        let window_size = window.window.get_size();
        ui::color(window_size, (0,0,0,255),(-window_size.0 as f32/2f32,window_size.1 as f32/2f32) , (window_size.0 as f32,55f32), 1);
        self.group.fast_draw(window); 
    }
}
