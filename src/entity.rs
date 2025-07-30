use std::any::{Any, TypeId};

use json::JsonValue;


pub trait Entity:Any{
    fn as_json(&self)->JsonValue{
        let type_id = TypeId::of::<Self>();
        JsonValue::String(format!("{:?}",type_id))
    }
}
