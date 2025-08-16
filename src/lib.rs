use std::{collections::HashMap, path::PathBuf, sync::LazyLock};





#[allow(unreachable_code)]
fn get_res_pak()->Pak{
    #[cfg(debug_assertions)]
    return main_pak::pak_path("res");
    //else
    return main_pak::Pak::load("main.pak"); 
}

pub struct ResLoader{
    pub model:HashMap<String,Model>
}
impl ResLoader{
    fn load(pak:&Pak)->Self{
        for name in pak.date.iter().map(|(name,value)|{name}){
            println!("{:?}",name);
        }
        let model = {
            let mut hashmap = HashMap::new();

            let group = pak.group_dir("/model/");
            
            for (path,item) in group{
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let value = Model::from_buffer(item);
                hashmap.insert(name, value);
            }
            hashmap
        };
        Self{
                model
        }
    }
}
pub static RES:LazyLock<ResLoader> = LazyLock::new(||{
    let pak = get_res_pak();
    ResLoader::load(&pak)
});



pub mod room;
pub mod entity;
pub mod object;


use gl_tools::draws::model::Model;
use main_pak::Pak;
pub use room::*;
pub use entity::*;
pub use object::*;
