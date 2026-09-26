use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::result::Result;
use std::fmt;
use super::parser_name;

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Int,
    Float,
    Bool,
    String,
    VectorInt,
    VectorFloat,
}

pub enum ParseError {
    LengthMismatch {
        expected: usize,
        received: usize,
    },

    InvalidValue {
        value: String,
        index: usize,
        value_type: ValueType
    },
}

pub enum ParseTypeError {
    InvalidType {
        value_type: String
    },
}

pub enum ValuesError {
    Occupied {
        hash: u64,
    },

    LengthMismatch {
        expected: usize,
        received: usize,
    },

    NullSize,

    DuplicateName {
        name: String,
        old_index: usize,
        index: usize,
    },

    EmptyName {
        index: usize,
    },

    InvalidName {
        name: String
    },
}

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    VectorInt(Vec<i64>),
    VectorFloat(Vec<f64>),
}

pub struct Schema {
    pub fields: Vec<Value>,
    pub map: HashMap<String, usize>,
}

pub struct Values {
    pub map: HashMap<u64, Schema>
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

impl fmt::Display for ParseError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::LengthMismatch { expected, received } => write!(f, "LengthMismatch: Expected: {}, Received: {}", expected, received),
            ParseError::InvalidValue { value, index, value_type } => write!(f, "InvalidValue: Value: {}, ValueType: {}, Index: {}", value, value_type, index)
        }
    }
}

impl fmt::Display for ValuesError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValuesError::LengthMismatch { expected, received } => write!(f, "LengthMismatch (Values and Names): Expected: {}, Received: {}", expected, received),
            ValuesError::NullSize => write!(f, "NullSize: Values and Names"),
            ValuesError::DuplicateName{ name, old_index, index} => write!(f, "DuplicateName: Name {}, with Index: {} already exists in index {}", name, index, old_index),
            ValuesError::InvalidName { name } => write!(f, "InvalidName: Name {} is invalid", name),
            ValuesError::EmptyName{index} => write!(f, "EmptyName: Name with index {} is empty", index),
            ValuesError::Occupied { hash } => write!(f, "Occupied: Hash {} already exists", hash),

        }
    }
}

impl Values {
    pub fn add_values(&mut self, hash: u64, values: Vec<Value>, names: Vec<String>) -> Result<(), ValuesError> {
        match self.map.entry(hash) {
            Entry::Occupied(_) => Err(ValuesError::Occupied { hash }),
            Entry::Vacant(m) => {
                if values.len() != names.len() {
                    return Err(ValuesError::LengthMismatch { expected: values.len(), received: names.len() })
                }

                if values.len() == 0 {
                    return Err(ValuesError::NullSize)
                }

                let mut schema: Schema = Schema { fields: Vec::with_capacity(values.len()), map: HashMap::with_capacity(values.len()) };


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
