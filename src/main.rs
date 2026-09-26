mod engine;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use engine::parser::VecValue;
use engine::types::Values;
use engine::types::ValueType;

use crate::engine::parser;
use crate::engine::types::ParseError;

fn main() -> Result<(), ()>{
    let name_struct = "Ids_1";

    let mut hasher = DefaultHasher::new();
    name_struct.hash(&mut hasher);
    let hash_res = hasher.finish();

    let str_values: Vec<&str> =  vec!["1", "[1,2,3,4,5]", "[1.5555, 255.55, 34.444]", "!"];
    let name_of_values: Vec<String> =  vec!["id", "sec_id", "third_3_id", "attention_555_id"].into_iter().map(|s| String::from(s)).collect();
    let type_of_values: Vec<ValueType> = vec![ValueType::Int, ValueType::VectorInt, ValueType::VectorFloat, ValueType::String];

    let mut vec_values: VecValue = VecValue { vec: Vec::new() };

    match vec_values.str_to_vec(str_values, type_of_values) {
        Err(e) => {
            eprint!("{}", e);
            return Err(())
        }
        Ok(_) => {}
    }

    println!("OK! VecValue is created!");

    let mut hash_val_map: Values = Values { map: HashMap::new() };

    match hash_val_map.add_values(hash_res, vec_values.vec, name_of_values) {
        Err(e) => {
            eprint!("{}", e);
            return Err(())
        }
        Ok(_) => {}
    }

    println!("OK! Insert into HashMap!");

    Ok(())
}
