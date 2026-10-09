use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::result::Result;
use std::num::NonZeroU32;
use std::fmt::{self};

use crate::engine::error_types::OperationError;

use super::error_types::{SetValueError, GetValueError};

use super::parser_name;
use super::error_types::ValuesError;
use super::operations::{ArithmeticOp, Operation, FieldRef};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i128),
    Float(f64),
    Bool(bool),
    String(String),
    VectorInt(Vec<i128>),
    VectorFloat(Vec<f64>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::Bool(_) => ValueType::Bool,
            Value::String(_) => ValueType::String,
            Value::VectorInt(_) => ValueType::VectorInt,
            Value::VectorFloat(_) => ValueType::VectorFloat,
        }
    }
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

    pub fn get_schema(&self, hash: &u64) -> Result<&Schema, GetValueError> {
        let schema = self.map.get(hash).ok_or(GetValueError::NonOccupiedHash { hash: *hash })?;

        Ok(schema)
    }

    pub fn get_mut_schema(&mut self, hash: &u64) -> Result<&mut Schema, GetValueError> {
        let schema = self.map.get_mut(hash).ok_or(GetValueError::NonOccupiedHash { hash: *hash })?;

        Ok(schema)
    }

    pub fn get_value(&mut self, hash: &u64, name: &str) -> Result<Value, GetValueError>  {
        let schema = self.get_schema(hash)?;

        let index = Table::find_name(name, schema)
            .ok_or(GetValueError::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        let value = schema.fields.get(index)
            .cloned()
            .ok_or_else(|| GetValueError::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        Ok(value)
    }

    pub fn get_mut_value(&mut self, hash: &u64, name: &str) -> Result<&mut Value, GetValueError>  {
        let schema = self.get_mut_schema(hash)?;

        let index = Table::find_name(name, schema)
            .ok_or(GetValueError::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        let value = schema.fields.get_mut(index)
            .ok_or_else(|| GetValueError::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        Ok(value)
    }

    pub fn set_value(&mut self, hash: &u64, name: &str, value: Value) -> Result<(), SetValueError>  {
        let schema = self.get_mut_schema(hash)?;

        if schema.ledger.is_some() {
            return Err(SetValueError::SetValueWithLedger { name: name.to_string(), hash: *hash });
        }

        let index = Table::find_name(name, schema)
             .ok_or(GetValueError::NonOccupiedName { name: name.to_string(), hash: *hash })?;

        let field = &mut schema.fields[index];

        if value.value_type() != field.value_type() {
            return Err(SetValueError::TypeMismatch { name: name.to_string(), hash: *hash, expected: field.value_type(), received: value.value_type() });
        }

        *field = value;

        Ok(())
    }

    pub fn apply(&mut self, operation: Operation) -> Result<(), OperationError> {
        match operation {
            Operation::Arith { target, op, amount } => {
                Ok(())
            },

            Operation::Transfer { from, to, amount } => {
                if from.hash == to.hash {
                    return Err(OperationError::EqualHashs { hash: from.hash })
                }

                let [schema_from, schema_to] = self.map.get_disjoint_mut([&from.hash, &to.hash]);

                let schema_from = schema_from.ok_or_else(|| GetValueError::NonOccupiedHash { hash: from.hash })?;
                let schema_to = schema_to.ok_or_else(|| GetValueError::NonOccupiedHash { hash: from.hash })?;

                if schema_from.ledger.is_none() {
                    return Err(OperationError::LedgerIsNone { name: from.name.to_string(), hash: from.hash })
                }

                if schema_to.ledger.is_none() {
                    return Err(OperationError::LedgerIsNone { name: to.name.to_string(), hash: to.hash })
                }

                if schema_from.ledger.unwrap() != schema_to.ledger.unwrap() {
                    return Err(OperationError::LedgerMismatch { expected: schema_from.ledger.unwrap(), received: schema_to.ledger.unwrap() })
                }

                let index_from = Table::find_name(from.name, schema_from)
                     .ok_or(GetValueError::NonOccupiedName { name: from.name.to_string(), hash: from.hash })?;
                let field_from = &mut schema_from.fields[index_from];

                let index_to = Table::find_name(from.name, schema_to)
                     .ok_or(GetValueError::NonOccupiedName { name: from.name.to_string(), hash: from.hash })?;
                let field_to = &mut schema_from.fields[index_to];

                Ok(())

            }
        }
    }
}
