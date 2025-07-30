use gl_tools::ui::{self, layout::{FloatLayout, LayoutPos, ListLayout}, object::{rc_refcell, UIbutton, UItext}, UIrender};




pub struct ToolBar{
    group:FloatLayout<ListLayout>
}
impl ToolBar{
    pub fn new()->Self{
        let mut list = ListLayout::new(LayoutPos::Right, 50f32, (0f32,0f32));
        list.add(UIbutton{check_click:false,text:UItext{text_color:(1f32,1f32,1f32,1f32),pos:(0f32,0f32),size:25i32,text:rc_refcell("Save".to_string())},action:Box::new(||{})}); 
        list.add(UIbutton{check_click:false,text:UItext{text_color:(1f32,1f32,1f32,1f32),pos:(0f32,0f32),size:25i32,text:rc_refcell("Load".to_string())},action:Box::new(||{})}); 
        
        let mut float = FloatLayout::new(list,(LayoutPos::Left,LayoutPos::Top));
        float.add_bound(LayoutPos::Left, 30f32);
        float.add_bound(LayoutPos::Top, 30f32);

        Self{
            group:float
        }
    }
}
impl UIrender for ToolBar{
    fn draw(&self)->Option<&gl_tools::gl_unit::FrameBuffer> {
        None
    }
    fn update(&mut self, window: &mut gl_tools::gl_unit::window::Window) {
        self.group.update(window);
    }
}
