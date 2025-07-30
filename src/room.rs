use gl_tools::glam::Mat4;
use json::{object, JsonValue};

use crate::{ Entity, Object};




pub struct Room{
    pub entity:Vec<Box<dyn Entity>>,
    pub object:Vec<Object>            
}
impl Into<JsonValue> for Room{
    fn into(self) -> JsonValue {
        let entity_list:Vec<JsonValue> = self.entity.iter().map(|monster|{monster.as_json()}).collect();
        object! {object:self.object,entity:entity_list}
    }
}
impl Room{
    pub fn from_json(json:JsonValue)->Self{
        todo!() 
    }
    pub fn new()->Self{
        Self{ entity: Vec::new(), object: Vec::new()}
    }
    pub fn draw(&self,mat:Mat4){
        for obj in self.object.iter(){
            obj.draw(mat);
        }
    }
}
