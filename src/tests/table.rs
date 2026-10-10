use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroU32;

use crate::engine::error_types::{GetValueError, SetValueError, ValuesError, ParseTypeError};
use crate::engine::types::{Table, Value, ValueType, Policy};

fn empty_table() -> Table {
    Table { map: HashMap::new() }
}

#[test]
fn table_add_values_none_ledger() {
    let mut table = empty_table();

    let values: Vec<Value> = vec![Value::Int(1), Value::UInt(1), Value::Float(1.0), Value::Bool(true), Value::String(String::from("Alice")), Value::VectorInt(vec![1, 2, 3, 4, 5]), Value::VectorFloat(vec![1.0, 2.0, 3.0, 4.0, 5.0])];
    //let types: Vec<ValueType> = vec![ValueType::Int, ValueType::UInt, ValueType::Float, ValueType::Bool, ValueType::String, ValueType::VectorInt, ValueType::VectorFloat];

    let names: Vec<&str> = vec!["_int", "_uint", "_float", "_bool", "_string", "_vec_int", "_vec_float"];

    let policy: Policy = Policy::ForeignInit;

    let ledger: Option<NonZeroU32> = None;

    assert_eq!(table.add_values(1, values, names, policy, ledger), Ok(()))
}
