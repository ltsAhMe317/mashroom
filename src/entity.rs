use std::any::{Any, TypeId};

use json::JsonValue;


pub trait Entity:Any{
    fn as_json(&self)->JsonValue;
    fn from_json(value:JsonValue)->Self where Self:Sized;
}
