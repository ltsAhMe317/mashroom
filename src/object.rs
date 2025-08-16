use gl_tools::{draws::model::{ModelPlayer, AABB, MODEL_PROGRAM}, glam::{vec3, Mat4, Vec3}};
use json::{JsonValue, object};

use crate::RES;

#[derive(Clone)]
pub enum HitMode {
    Model,
    Box(Vec<(f32, f32, f32, f32, f32)>),
}

pub struct ObjectOffset{
    position:Vec3,
    scale:Vec3,
    rotate:Vec3, 
}
impl ObjectOffset{
    pub fn new()->ObjectOffset{
        ObjectOffset { position:vec3(0f32, 0f32, 0f32) , scale: vec3(1f32, 1f32, 1f32), rotate: vec3(0f32, 0f32, 0f32) }
    }
    pub fn pos(mut self,x:f32,y:f32,z:f32)->Self{
        self.position = vec3(x, y, z);
        self
    }
    pub fn scale(mut self,x:f32,y:f32,z:f32)->Self{
        self.scale = vec3(x, y, z);
        self
    }
    pub fn rotate(mut self,x:f32,y:f32,z:f32)->Self{
        self.rotate = vec3(x, y, z);
        self
    }
}

impl Clone for Object<'_>{
    fn clone(&self) -> Self {
        Self{
            model: self.model.clone(),
            player: RES.model.get(&self.model).unwrap().player(),            
            mat: self.mat,
            hit_box: self.hit_box.clone(),
        }
    }
}
pub struct Object<'a> {
    pub model: String,
    pub player:ModelPlayer<'a>,
    pub mat: Mat4,
    pub hit_box: HitMode,
}
impl Object<'_> {
    pub fn new(model: &str, hit_box: HitMode,mat:Mat4) -> Self {
        Self {
            model: model.to_string(),
            mat,
            hit_box,
            player: RES.model.get(model).unwrap().player(),
        }
    }
    pub fn mat(mut self,layout:ObjectOffset)->Self{
        let mat =Mat4::from_rotation_x(layout.rotate.z)* Mat4::from_rotation_x(layout.rotate.y)* Mat4::from_rotation_x(layout.rotate.x)*  Mat4::from_scale(layout.scale) * Mat4::from_translation(layout.position);
        self.mat = mat;
        self
    }
    pub fn draw(&self, mat: Mat4) {
        MODEL_PROGRAM.bind();
        MODEL_PROGRAM.put_matrix_name(mat, "project_mat");
        MODEL_PROGRAM.put_matrix_name(self.mat, "model_mat");
        self.player.draw(RES.model.get(&self.model).unwrap(),&MODEL_PROGRAM);
    }
    pub fn update(&mut self,delta:f32){
        self.player.time_add(delta);
        self.player.load(RES.model.get(&self.model).unwrap());
    }
}
impl Into<JsonValue> for Object<'_> {
    fn into(self) -> JsonValue {
        object! {model:self.model,mat:self.mat.to_cols_array().to_vec()}
    }
}
