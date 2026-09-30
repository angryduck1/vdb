use super::types::ValueType;
use std::fmt;

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

pub enum ParseLedgerError {
    InvalidValue {
        value: String
    },

    ZeroValue,
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

impl fmt::Display for ParseError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::LengthMismatch { expected, received } => write!(f, "LengthMismatch: Expected: {}, Received: {}", expected, received),
            ParseError::InvalidValue { value, index, value_type } => write!(f, "InvalidValue: Value: {}, ValueType: {}, Index: {}", value, value_type, index)
        }
    }
}

impl fmt::Display for ParseLedgerError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseLedgerError::InvalidValue { value } => write!(f, "InvalidType: Fail Convert {} to Ledger", value),
            ParseLedgerError::ZeroValue => write!(f, "ZeroValue: Fail Convert u32 to Ledger"),
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
