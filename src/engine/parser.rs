use super::types::Value;
use super::types::ValueType;
use super::types::ParseError;
use super::types::ParseTypeError;
use std::str::FromStr;

pub struct VecValue {
    pub vec: Vec<Value>
}

impl FromStr for ValueType {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "int" | "i64" => Ok(Self::Int),
            "float" | "f64" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            "string" | "str" => Ok(Self::String),
            "vec_int" | "vec_i64" => Ok(Self::VectorInt),
            "vec_float" | "vec_f64" => Ok(Self::VectorFloat),
            other => Err(ParseTypeError::InvalidType{value_type: other.to_string()}),
        }
    }
}

impl VecValue {
    pub fn str_to_vec(&mut self, values: Vec<&str>, values_types: Vec<ValueType>) -> Result<(), ParseError> {
        if values.len() != values_types.len() {
            return Err(ParseError::LengthMismatch { expected: values.len(), received: values_types.len() })
        }

        for (i, (val, val_type)) in values.iter().zip(values_types.iter()).enumerate() {
            match val_type {
                ValueType::Int => self.vec.push(Value::Int(
                        val.parse::<i64>().map_err(|_| ParseError::InvalidValue{ value: val.to_string(), index: i, value_type: *val_type }
                )?)),

                ValueType::Float => self.vec.push(Value::Float(
                        val.parse::<f64>().map_err(|_| ParseError::InvalidValue{ value: val.to_string(), index: i, value_type: *val_type }
                )?)),

                ValueType::Bool => self.vec.push(Value::Bool(
                        val.parse::<bool>().map_err(|_| ParseError::InvalidValue{ value: val.to_string(), index: i, value_type: *val_type }
                )?)),

                ValueType::String => {
                    self.vec.push(Value::String(val.to_string()));
                }

                ValueType::VectorInt => {
                    let v = val.trim_start_matches('[').trim_end_matches(']').split(',').map(|v| v.trim().trim_ascii_start().trim_ascii_end().parse::<i64>()).collect::<Result<Vec<i64>, _>>().map_err(|_| ParseError::InvalidValue{ value: val.to_string(), index: i, value_type: *val_type})?;
                    self.vec.push(Value::VectorInt(v));
                }

                ValueType::VectorFloat => {
                    let v = val.trim_start_matches('[').trim_end_matches(']').split(',').map(|v| v.trim().trim_ascii_start().trim_ascii_end().parse::<f64>()).collect::<Result<Vec<f64>, _>>().map_err(|_| ParseError::InvalidValue{ value: val.to_string(), index: i, value_type: *val_type})?;
                    self.vec.push(Value::VectorFloat(v));
                }
            }
        }

        Ok(())
    }

    pub fn get_vec(&self) -> &Vec<Value> {
        &self.vec
    }
}
