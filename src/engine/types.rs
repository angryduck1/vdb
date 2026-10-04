use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::result::Result;
use std::num::NonZeroU32;
use std::fmt::{self, write};

use crate::engine::error_types::GetValue;

use super::parser_name;
use super::error_types::ValuesError;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i128),
    Float(f64),
    Bool(bool),
    String(String),
    VectorInt(Vec<i128>),
    VectorFloat(Vec<f64>),
}

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Int,
    Float,
    Bool,
    String,
    VectorInt,
    VectorFloat,
}

pub enum Policy {
    ForeignInit,
    Inline
}

impl Value {
    pub fn zero_int() -> Self{Value::Int(0)}
    pub fn zero_float() -> Self{Value::Float(0.0)}
    pub fn zero_bool() -> Self{Value::Bool(false)}
    pub fn empty_string() -> Self{Value::String(String::new())}
    pub fn empty_vecint() -> Self{Value::VectorInt(Vec::new())}
    pub fn empty_vecfloat() -> Self{Value::VectorFloat(Vec::new())}
}


impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Int => write!(f, "Int"),
            ValueType::Float => write!(f, "Float"),
            ValueType::Bool => write!(f, "Bool"),
            ValueType::String => write!(f, "String"),
            ValueType::VectorInt => write!(f, "VectorInt"),
            ValueType::VectorFloat => write!(f, "VectorFloat"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v.to_string()),
            Value::Float(v) => write!(f, "{}", v.to_string()),
            Value::Bool(v) => write!(f, "{}", v.to_string()),
            Value::String(v) => write!(f, "{}", v),
            Value::VectorInt(v) => write!(f, "{}", v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")),
            Value::VectorFloat(v) => write!(f, "{}", v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))
        }
    }
}

pub struct Schema {
    pub fields: Box<[Value]>,
    pub policy: Policy,
    pub ledger: Option<NonZeroU32>,
    names: Box<str>,
}

pub struct Table {
    pub map: HashMap<u64, Schema>
}

impl Table {
    pub fn add_values(&mut self, hash: u64, values: Vec<Value>, names: Vec<&str>, policy: Policy, ledger: Option<NonZeroU32>) -> Result<(), ValuesError> {
        match self.map.entry(hash) {
            Entry::Occupied(_) => Err(ValuesError::Occupied { hash }),
            Entry::Vacant(m) => {
                if values.len() != names.len() {
                    return Err(ValuesError::LengthMismatch { expected: values.len(), received: names.len() })
                }

                if values.len() == 0 {
                    return Err(ValuesError::NullSize)
                }

                for (i, name) in names.iter().enumerate() {
                    parser_name::is_valid_name(i, name)?
                }

                let join_names = names.join("|");

                let mut unique_names = HashSet::with_capacity(names.len());

                for (i, name) in names.iter().enumerate() {
                    parser_name::is_valid_name(i, name)?;

                    if !unique_names.insert(*name) {
                        return Err(ValuesError::DuplicateName { name: name.to_string() });
                    }
                }

                let schema: Schema = Schema { fields: values.into_boxed_slice(), policy: policy, ledger: ledger, names: join_names.into_boxed_str()  };

                m.insert(schema);

                Ok(())
            }
        }
    }

    fn find_name(name: &str, schema: &Schema) -> Option<usize> {
        schema.names.split("|").position(|n| n == name)
    }

    pub fn get_value(&mut self, hash: &u64, name: &str) -> Result<Value, GetValue>  {
        let schema = self.map.get(hash).ok_or(GetValue::NonOccupiedHash { hash: *hash })?;

        let index: usize = Table::find_name(name, schema)
            .ok_or(GetValue::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        let value: Value = schema.fields.get(index)
            .cloned()
            .ok_or_else(|| GetValue::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        Ok(value)
    }
}
