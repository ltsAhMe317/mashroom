use std::{any::{Any, TypeId}, collections::HashMap};

use gl_tools::{gl::MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS, glam::Vec3};
#[macro_export]
macro_rules! get_component {
    ($map:expr,$type:ident) => {
        {
            $map.get_component(TypeId::of::<$type>()).as_any().downcast_ref::<$type>().unwrap()
        }
    };
}
#[macro_export]
macro_rules! get_component_mut {
    ($map:expr,$type:ident) => {
        {
            $map.get_component_mut(TypeId::of::<$type>()).as_any_mut().downcast_mut::<$type>().unwrap()
        }
    };
}

#[macro_export]
macro_rules! component_map {
    ($($value:expr),*) => {
        {
            use std::collections::HashMap;
        let mut map:HashMap<TypeId,Box<dyn Component>> = HashMap::new();
        {
            $(map.add_component(Box::new($value));)*
        }
        map
        }
    };
}
macro_rules! component {
    ($($name:ident),*) => {
        $(impl Component for $name{
        fn as_any(&self)->&dyn Any{
            self
        }
    fn as_any_mut(&mut self)->&mut dyn Any{
        self
    }
        })*
    };
}
pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait ComponentList {
    fn add_component(&mut self,component:Box<dyn Component>) ;
    fn get_component(&self,id:TypeId)->&dyn Component;
    fn get_component_mut(&mut self,id:TypeId)->&mut dyn Component;

}
impl ComponentList for  HashMap<TypeId,Box<dyn Component>> {
    fn add_component(&mut self,component:Box<dyn Component>)  {
        self.insert(component.as_any().type_id(), component);
    }
    fn get_component(&self,id:TypeId)->&dyn Component {
        self.get(&id).unwrap().as_ref()
    }
    fn get_component_mut(&mut self,id:TypeId)->&mut dyn Component {
        self.get_mut(&id).unwrap().as_mut()
    }
}

pub type ComponentMap = HashMap<TypeId,Box<dyn Component>>;

component!(Hp);
pub struct Hp {
    pub value: f32,
}

component!(Pos);
pub struct Pos{
    pub value:Vec3
}

