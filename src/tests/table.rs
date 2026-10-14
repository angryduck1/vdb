use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::NonZeroU32;

use crate::engine::error_types::{GetValueError, ParseLedgerError, ParseTypeError, SetValueError, ValuesError};
use crate::engine::parser::VecValue;
use crate::engine::parser_ledger::parse_ledger;
use crate::engine::types::{Table, Value, ValueType, Policy};

pub const DEF_NAMES: [&str; 7] = ["_int", "_uint", "_float", "_bool", "_string", "_vec_int", "_vec_float"];
pub const DEF_TYPES: [ValueType; 7] = [ValueType::Int, ValueType::UInt, ValueType::Float, ValueType::Bool, ValueType::String, ValueType::VectorInt, ValueType::VectorFloat];
const DEF_HASH: u64 = 1;

fn empty_table() -> Table {
    Table { map: HashMap::new() }
}

fn zero_init(values_types: Vec<ValueType>) -> Vec<Value> {
    let mut values = VecValue{vec: Vec::new()};

    values.zero_init(values_types);

    values.vec
}

fn default_values(values_types: Vec<ValueType>) -> Vec<Value> {
    zero_init(values_types)
}

pub fn default_init_table(ledger: Option<NonZeroU32>) -> Table {
    let mut table = empty_table();

    let values = default_values(DEF_TYPES.to_vec());

    table.add_values(DEF_HASH, values, DEF_NAMES.to_vec(), Policy::Inline, ledger).unwrap();

    table
}

pub fn default_add_values(table: &mut Table, hash: u64, ledger: Option<NonZeroU32>) {
    let values = default_values(DEF_TYPES.to_vec());

    table.add_values(hash, values, DEF_NAMES.to_vec(), Policy::Inline, ledger).unwrap();
}

#[test]
fn table_add_values_none_ledger() {
    let mut table = empty_table();

    assert_eq!(table.add_values(DEF_HASH, default_values(DEF_TYPES.to_vec()), DEF_NAMES.to_vec(), Policy::Inline, None),
        Ok(())
    );

    assert_eq!(table.get_value(&DEF_HASH, "_int"), Ok(Value::Int(0)));

    assert_eq!(
        table.add_values(DEF_HASH, default_values(DEF_TYPES.to_vec()), DEF_NAMES.to_vec(), Policy::ForeignInit, None),
        Err(ValuesError::Occupied { hash: 1 })
    );

    assert_eq!(
        table.add_values(2, default_values(DEF_TYPES.to_vec()), vec!["_int", "_int", "_float", "_bool", "_string", "_vec_int", "_vec_float"], Policy::Inline, None),
        Err(ValuesError::DuplicateName { name: String::from("_int") })
    );

    assert_eq!(
        table.add_values(3, default_values(DEF_TYPES.to_vec()), vec!["_int"], Policy::Inline, None),
        Err(ValuesError::LengthMismatch { expected: DEF_TYPES.len(), received: 1 })
    );

    assert_eq!(
        table.add_values(4, vec![], vec![], Policy::ForeignInit, None),
        Err(ValuesError::NullSize)
    );

    assert_eq!(
        table.add_values(5, default_values(DEF_TYPES.to_vec()), vec!["_int", "", "_float", "_bool", "_string", "_vec_int", "_vec_float"], Policy::ForeignInit, None),
        Err(ValuesError::EmptyName { index: 1 })
    );

    assert_eq!(
        table.add_values(6, default_values(DEF_TYPES.to_vec()), vec!["_int", "1", "_float", "_bool", "_string", "_vec_int", "_vec_float"], Policy::ForeignInit, None),
        Err(ValuesError::InvalidName { name: String::from("1") })
    );
}

#[test]
fn table_set_value_none_ledger() {
    let mut table = default_init_table(None);

    let values: [Value; 7] = [Value::Int(1), Value::UInt(1), Value::Float(1.0), Value::Bool(true), Value::String(String::from("meow")), Value::VectorInt(vec![1,2,3,4,5]), Value::VectorFloat(vec![1.0,2.0,3.0,4.0,5.0])];

    for (n, v) in DEF_NAMES.to_vec().iter().zip(values) {
        assert_eq!(table.set_value(&DEF_HASH, *n, v), Ok(()));
    }

    assert_eq!(table.set_value(&DEF_HASH, DEF_NAMES[0], Value::UInt(1)), Err(SetValueError::TypeMismatch { name: DEF_NAMES[0].to_string(), hash: DEF_HASH, expected: DEF_TYPES[0], received: ValueType::UInt }));
}

#[test]
fn table_set_value_with_ledger() {
    let mut table = default_init_table(Some(parse_ledger("1").unwrap()));

    let values: [Value; 7] = [Value::Int(1), Value::UInt(1), Value::Float(1.0), Value::Bool(true), Value::String(String::from("meow")), Value::VectorInt(vec![1,2,3,4,5]), Value::VectorFloat(vec![1.0,2.0,3.0,4.0,5.0])];

    for (n, v) in DEF_NAMES.to_vec().iter().zip(values) {
        assert_eq!(table.set_value(&DEF_HASH, *n, v), Err(SetValueError::SetValueWithLedger { name: n.to_string(), hash: DEF_HASH }));
    }
}

#[test]
fn table_get_value() {
    let mut table = default_init_table(None);

    let def_values = default_values(DEF_TYPES.to_vec());

    for (n, v) in DEF_NAMES.to_vec().iter().zip(def_values) {
        assert_eq!(table.get_value(&DEF_HASH, n), Ok(v));
    }

    assert_eq!(table.get_value(&DEF_HASH, "meow"), Err(GetValueError::NonOccupiedName { name: String::from("meow"), hash: DEF_HASH }));

    assert_eq!(table.get_value(&2u64, "_int"), Err(GetValueError::NonOccupiedHash { hash: 2u64 }));
}
