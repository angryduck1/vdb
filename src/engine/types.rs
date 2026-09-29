use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::result::Result;
use std::num::NonZeroU32;
use std::fmt;

use super::parser_name;
use super::error_types::ValuesError;

#[derive(Debug)]
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

pub struct Schema {
    pub fields: Vec<Value>,
    pub policy: Policy,
    pub ledger: Option<NonZeroU32>,
    pub map: HashMap<String, usize>,
}

pub struct Table {
    pub map: HashMap<u64, Schema>
}

impl Table {
    pub fn add_values(&mut self, hash: u64, values: Vec<Value>, names: Vec<String>, policy: Policy, ledger: Option<NonZeroU32>) -> Result<(), ValuesError> {
        match self.map.entry(hash) {
            Entry::Occupied(_) => Err(ValuesError::Occupied { hash }),
            Entry::Vacant(m) => {
                if values.len() != names.len() {
                    return Err(ValuesError::LengthMismatch { expected: values.len(), received: names.len() })
                }

                if values.len() == 0 {
                    return Err(ValuesError::NullSize)
                }

                let mut schema: Schema = Schema { fields: Vec::with_capacity(values.len()), map: HashMap::with_capacity(values.len()), policy: policy, ledger: ledger  };


                for (i, name) in names.iter().enumerate() {
                    parser_name::is_valid_name(i, name)?
                }

                for (i, (val, name)) in values.into_iter().zip(names).into_iter().enumerate() {
                    if let Some(old_index) = schema.map.insert(name.clone(), i) {
                        return Err(ValuesError::DuplicateName { name, old_index, index: i })
                    }

                    schema.fields.push(val)
                }

                m.insert(schema);

                Ok(())
            }
        }
    }
}
