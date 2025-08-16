use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    rc::Rc,
};
use rfd::FileDialog;
use gl_tools::{gl::GetQueryBufferObjectuiv, glam::Mat4, ui::{
    self, layout::{BoundLayout, FloatLayout, LayoutPos, ListLayout, WindowLayout}, object::{rc_refcell, UIbutton, UIinput, UIkeep, UItext}, KeyStream, UIlayout, UIrender
}};
use mashroom::{entity_maker, HitMode, Object, ObjectOffset};

use crate::Room;

pub struct ToolBar {
    group: FloatLayout<BoundLayout<ListLayout>>,
}
impl ToolBar {
    pub fn new(room: &Rc<RefCell<Option<Room>>>) -> (Self,Rc<RefCell<bool>>) {
        let mut list = ListLayout::new(LayoutPos::Right, 50f32, (0f32, 0f32));
        {
            let room = room.clone();
            list.add(UIbutton {
                check_click: false,
                text: UItext {
                    color: (1f32, 1f32, 1f32, 1f32),
                    pos: (0f32, 0f32),
                    text_size: 25i32,
                    text: rc_refcell("Save".to_string()),
                },
                action: Box::new(move || {
                    let file = FileDialog::new().set_title("save");
                    if let Some(file) = file.save_file()
                    {
                        fs::write(
                            file,
                            format!("{}", room.borrow().as_ref().unwrap().as_json()),
                        )
                        .unwrap();
                    }
                }),
            });
        }
        {
            let room = room.clone();
            list.add(UIbutton {
                check_click: false,
                text: UItext {
                    color: (1f32, 1f32, 1f32, 1f32),
                    pos: (0f32, 0f32),
                    text_size: 25i32,
                    text: rc_refcell("Load".to_string()),
                },
                action: Box::new(move|| {
                    if let Some(path) = FileDialog::new().pick_file(){
                        *room.borrow_mut() = Some(Room::from_json(json::from(fs::read_to_string(path).unwrap())));
                    }
                }),
            });
        }
        let (obj,key) = UIkeep::new("update");
        list.add(obj);
        
        let mut list = BoundLayout {
            pos: (0f32, 0f32),
            bound: HashMap::new(),
            obj: list,
        };
        list.add_bound(LayoutPos::Left, 30f32);
        list.add_bound(LayoutPos::Top, 30f32);
        list.add_bound(LayoutPos::Bottom, 20f32);

        let float = FloatLayout::new(list, (LayoutPos::Left, LayoutPos::Top));

        (Self { group: float },key)
    }
}
impl UIrender for ToolBar {
    fn draw(&self) -> Option<&gl_tools::gl_unit::FrameBuffer> {
        None
    }
    fn update(
        &mut self,
        window: &mut gl_tools::gl_unit::window::Window,
        key_stream: &mut KeyStream,
    ) {
        self.group.update(window, key_stream);
    }
    fn fast_draw(&self, window: &mut gl_tools::gl_unit::window::Window) {
        let window_size = window.window.get_size();
        ui::color(
            window_size,
            (0, 0, 0, 255),
            (-window_size.0 as f32 / 2f32, window_size.1 as f32 / 2f32),
            (window_size.0 as f32, self.group.obj.size().1),
            1,
        );
        self.group.fast_draw(window);
    }
}
pub fn new_object(room:Rc<RefCell<Option<Room>>>)->WindowLayout<ListLayout>{
        let (name_input_obj, name_input) = UIinput::new((0f32, 0f32));
        let (x_y_z_group, x_input, y_input, z_input) = {
            let mut list = ListLayout::new(LayoutPos::Right, 10f32, (0f32, 0f32));
            list.add(UItext::new("position"));
            let (x_input_obj, x_input) = UIinput::new((0f32, 0f32));
            let (y_input_obj, y_input) = UIinput::new((0f32, 0f32));
            let (z_input_obj, z_input) = UIinput::new((0f32, 0f32));
            list.add(x_input_obj);
            list.add(y_input_obj);
            list.add(z_input_obj);
            (list, x_input, y_input, z_input)
        };
        let mut list = ListLayout::new(LayoutPos::Bottom, 20f32, (0f32, 0f32));
        list.add(name_input_obj);
        list.add(x_y_z_group);
        *name_input.borrow_mut() = "name".to_string();
        *x_input.borrow_mut() = "0".to_string();
        *y_input.borrow_mut() = "0".to_string();
        *z_input.borrow_mut() = "0".to_string();
        let room = room.clone();
        {
            let x_input = x_input.clone();
            let y_input = y_input.clone();
            let z_input = z_input.clone();
        let button= UIbutton{ check_click:false, text:UItext::new("new"), action:Box::new(move||{
            let object = Object::new(name_input.borrow().as_str(),HitMode::Model,Mat4::IDENTITY).mat(ObjectOffset::new().pos(x_input.borrow().as_str().parse().unwrap(), y_input.borrow().as_str().parse().unwrap(), z_input.borrow().as_str().parse().unwrap()));
            room.borrow_mut().as_mut().unwrap().object.push(object);
        }) };
        list.add(button);
        }
        WindowLayout::new((0f32, 0f32), "Object", 25, list)
    }

pub fn new_entity(room:Rc<RefCell<Option<Room>>>)->WindowLayout<ListLayout>{
        let (name_input_obj, name_input) = UIinput::new((0f32, 0f32));
        
        let mut list = ListLayout::new(LayoutPos::Bottom, 20f32, (0f32, 0f32));
        list.add(name_input_obj);
        *name_input.borrow_mut() = "name".to_string();

        let room = room.clone();
        {
            
        let button= UIbutton{ check_click:false, text:UItext::new("new"), action:Box::new(move||{
            room.borrow_mut().as_mut().unwrap().entity.push(entity_maker(&json::parse(name_input.borrow().as_str()).unwrap()));
         }) };
        list.add(button);
        }
        WindowLayout::new((0f32, 0f32), "Entity", 25, list)
    }

