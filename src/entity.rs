pub mod component;



use std::any::{Any, TypeId};

use gl_tools::{draws::model::MODEL_PROGRAM, gl_unit::window::Window, glam::{vec3, Mat4, Vec3}};
use json::{object, JsonValue};

use crate::{component::{Component, ComponentList, ComponentMap, Hp, Pos}, component_map, get_component, get_component_mut, Object, RES};

pub trait EntityName{
    fn name(&self)->&'static str;
}
macro_rules! EntityNameGen {
    ($($name:ident),*) => {
        $(impl EntityName for $name{
            fn name(&self)->&'static str{
                stringify!($name)
            }
        })*
        pub fn entity_maker(value:&JsonValue)->Box<dyn Entity>{
            Box::new(match value["name"].to_string().as_str(){
                $(stringify!($name) => $name::from_json(value),)*
                _ =>{todo!()}
            }) 
        }
    };
}
EntityNameGen!(Tree);

pub trait Entity:Any+EntityName{
    fn as_json(&self)->JsonValue;
    fn from_json(value:&JsonValue)->Self where Self:Sized;
    fn update(&mut self,window:&mut Window)->Box<dyn FnOnce(usize,&mut Vec<Box<dyn Entity>>,&Vec<Object>)>;
    fn draw(&self,window:&Window,camera:Mat4);
    fn is_clear(&self)->bool{
        false
    }
    fn components_ref(&self)->&dyn ComponentList;
    fn components_mut(&mut self)->&mut dyn ComponentList;
}

pub struct Tree{
    components:ComponentMap,
}
impl Entity for Tree{
    fn as_json(&self)->JsonValue {
        let pos = get_component!(self.components,Pos).value;
        object! {name:"Tree",pos:[pos.x,pos.y,pos.z]}
    }
    fn from_json(value:&JsonValue)->Self where Self:Sized {
        println!("{value}");
        let binding = value["pos"].clone();
        let mut value = binding.members();
        Self{
            components: component_map!(Hp{value:100f32},Pos{value:vec3(value.next().unwrap().as_f32().unwrap(), value.next().unwrap().as_f32().unwrap(), value.next().unwrap().as_f32().unwrap())}),
        }
    }
    fn draw(&self,window:&Window,camera:Mat4) {
        MODEL_PROGRAM.bind();
        MODEL_PROGRAM.put_matrix_name(camera, "project_mat");
        dbg!(get_component!(self.components,Pos).value);
        MODEL_PROGRAM.put_matrix_name(Mat4::from_translation(get_component!(self.components,Pos).value), "model_mat");
        // RES.model.get("tree.glb").unwrap().draw(&MODEL_PROGRAM);
    }

    fn update(&mut self,window:&mut Window)->Box<dyn FnOnce(usize,&mut Vec<Box<dyn Entity>>,&Vec<Object>)> {
        let delta = window.delta_count.time_count as f32;
        dbg!(
            get_component_mut!(self.components_mut(),Pos).value.y = delta.sin()

        );
        Box::new(move|id,entity,obj|{
            let entity =entity.get_mut(id).unwrap().as_mut() as &mut dyn Any;
            get_component_mut!(entity.downcast_mut::<Tree>().unwrap().components_mut(),Pos).value.y = delta.sin();
            get_component_mut!(entity.downcast_mut::<Tree>().unwrap().components_mut(),Pos).value.x = delta.cos();
        })
    }
    fn components_ref(&self)->&dyn ComponentList {
        &self.components
    }
    fn components_mut(&mut self)->&mut dyn ComponentList {
        &mut self.components
    }
}
