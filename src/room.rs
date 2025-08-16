use std::ops::Deref;

use gl_tools::{
    gl_unit::{self, define::VertexArrayAttribPointerGen, program::PROGRAM3D_ONE_COLOR, window::Window}, glam::Mat4, VAO_MUT, VAO_STATIC, VERTEX_BIG_MUT
};
use json::{JsonValue, object};

use crate::{entity_maker, Entity, Object};

pub struct Room {
    pub entity: Vec<Box<dyn Entity>>,
    pub object: Vec<Object<'static>>,
}
impl Room {
    pub fn as_json(&self) -> JsonValue {
        let entity_list: Vec<JsonValue> = self
            .entity
            .iter()
            .map(|monster| monster.as_json())
            .collect();
        let object_list: Vec<JsonValue> = self
            .object
            .iter()
            .map(|object| object.clone().into())
            .collect();

        object! {object:object_list,entity:entity_list}
    }
}
impl Room {
    pub fn from_json(mut json: JsonValue) -> Self {
        let mut room = Room::new();
        let mut object = json["object"].take();
        for obj in object.members_mut() {
            let mat = {
                let mut vec = [0f32; 16];
                let mat_vec = obj["mat"].take();
                for (index, value) in mat_vec.members().enumerate() {
                    if let JsonValue::Number(value) = value {
                        vec[index] = Into::<f32>::into(*value);
                    }
                }
                Mat4::from_cols_array(&vec)
            };
            room.object.push(Object::new(
                obj["model"].take().as_str().unwrap(),
                crate::HitMode::Model,
                mat,
            ));
        }
        let mut entity = json["entity"].take();
        for entity in entity.members_mut() {
            room.entity.push(entity_maker(entity));
        }

        room
    }
    pub fn new() -> Self {
        Self {
            entity: Vec::new(),
            object: Vec::new(),
        }
    }
    pub fn draw(&self,window:&Window,camera: Mat4) {
        for obj in self.object.iter() {
            obj.draw(camera);
            // gl_unit::polygon_mode(
            //     gl_unit::define::Face::FrontAndBack,
            //     gl_unit::define::PolygonMode::Line(3f32),
            // );
            // PROGRAM3D_ONE_COLOR.bind();
            // PROGRAM3D_ONE_COLOR.put_matrix_name(camera, "project_mat");
            // PROGRAM3D_ONE_COLOR.put_matrix_name(Mat4::IDENTITY, "model_mat");
            // PROGRAM3D_ONE_COLOR.put_vec4(
            //     [1f32, 0f32, 0f32, 1f32],
            //     PROGRAM3D_ONE_COLOR.get_uniform("color"),
            // );
            // VERTEX_BIG_MUT.sub_data(&obj.aabb.as_vertexs(), 0);
            // VAO_MUT.bind(|vao| {
            //     vao.pointer(
            //         VERTEX_BIG_MUT.deref(),
            //         VertexArrayAttribPointerGen::new::<f32>(0, 3),
            //     );
            //     vao.draw_arrays(gl_unit::define::DrawMode::Quads, 0, 4 * 6);
            // });
            // gl_unit::polygon_mode(
            //     gl_unit::define::Face::FrontAndBack,
            //     gl_unit::define::PolygonMode::Fill,
            // );
        }
        for entity in self.entity.iter(){
            entity.draw(window,camera);
        }
    }
    pub fn update(&mut self,window:&mut Window){
        for obj in self.object.iter_mut(){
            obj.update(window.delta_count.delta as f32);
        }
        
        
        let mut action = Vec::new();
        for (id,entity) in self.entity.iter_mut().enumerate(){
           action.push((id,entity.update(window)));
        }
        for (id,action) in action{
            action(id,&mut self.entity,&self.object);
        }
        //genius
        let vec = Vec::with_capacity(self.entity.len());
        for entity in std::mem::replace(&mut self.entity, vec).into_iter(){
            if !entity.is_clear(){
                self.entity.push(entity);
            }
        }        
    }
}
