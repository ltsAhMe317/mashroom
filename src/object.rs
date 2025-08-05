use gl_tools::{draws::model::MODEL_PROGRAM, glam::Mat4};
use json::{JsonValue, object};

use crate::RES;

#[derive(Clone, Copy)]
pub enum HitMode {
    Model,
    Box(f32, f32, f32, f32, f32),
}
#[derive(Clone)]
pub struct Object {
    model: String,
    mat: Mat4,
    hit_box: HitMode,
}
impl Object {
    pub fn new(model: &str, hit_box: HitMode) -> Self {
        Self {
            model: model.to_string(),
            mat: Mat4::IDENTITY,
            hit_box,
        }
    }
    pub fn draw(&self, mat: Mat4) {
        MODEL_PROGRAM.bind();
        MODEL_PROGRAM.put_matrix_name(mat, "project_mat");
        MODEL_PROGRAM.put_matrix_name(Mat4::IDENTITY, "model_mat");
        RES.model.get(&self.model).unwrap().draw(&MODEL_PROGRAM);
    }
}
impl Into<JsonValue> for Object {
    fn into(self) -> JsonValue {
        object! {model:self.model,mat:self.mat.to_cols_array().to_vec()}
    }
}
