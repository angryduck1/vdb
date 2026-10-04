mod engine;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use engine::parser::VecValue;
use engine::types::{ValueType, Policy};

use crate::engine::types::{Table};

fn main() -> Result<(), ()>{
    let name_struct = "Ids_1";

    let mut hasher = DefaultHasher::new();
    name_struct.hash(&mut hasher);
    let hash_res = hasher.finish();

    //let str_values: Vec<&str> =  vec!["1", "1,2,3,4,5]", "[1.5555, 255.55, 34.444]", "!"];
    let name_of_values: Vec<&str> =  vec!["id", "sec_id", "third_3_id", "attention_555_id"];
    let type_of_values: Vec<ValueType> = vec![ValueType::Int, ValueType::VectorInt, ValueType::VectorFloat, ValueType::String];

    let mut vec_values: VecValue = VecValue { vec: Vec::new() };

    vec_values.zero_init(type_of_values);

    println!("OK! VecValue is created!");

    let mut hash_val_map: Table = Table { map: HashMap::new() };

    match hash_val_map.add_values(hash_res, vec_values.vec, name_of_values, Policy::Inline, None) {
        Err(e) => {
            eprint!("{}", e);
            return Err(())
        }
        Ok(_) => {}
    }

    println!("OK! Insert into HashMap!");

    match hash_val_map.get_value(&hash_res, "id") {
        Err(e) => {
            eprint!("{}", e);
            return Err(())
        }
        Ok(v) => {
            println!("{}", v);
        }
    }

    Ok(())
}
