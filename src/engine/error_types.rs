use super::types::ValueType;
use std::fmt::{self, write};
use std::num::NonZeroU32;

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
    },

    EmptyName {
        index: usize,
    },

    InvalidName {
        name: String
    },
}

pub enum GetSetValueError {
    NonOccupiedHash {
        hash: u64,
    },

    NonOccupiedName {
        name: String,
        hash: u64,
    },

    SetValueWithLedger {
        name: String,
        hash: u64,
    },

    TypeMismatch {
        name: String,
        hash: u64,
        expected: ValueType,
        received: ValueType,
    }
}

pub enum OperationError {
    Access(GetSetValueError),

    TypeMismatch {
        name: String,
        hash: u64,
        expected: ValueType,
        received: ValueType,
    },

    LedgerMismatch {
        expected: NonZeroU32,
        received: NonZeroU32,
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
            ValuesError::DuplicateName{ name } => write!(f, "DuplicateName: Name {}, already exists", name),
            ValuesError::InvalidName { name } => write!(f, "InvalidName: Name {} is invalid", name),
            ValuesError::EmptyName{index} => write!(f, "EmptyName: Name with index {} is empty", index),
            ValuesError::Occupied { hash } => write!(f, "Occupied: Hash {} already exists", hash),

        }
    }
}

impl fmt::Display for GetSetValueError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetSetValueError::NonOccupiedHash { hash } => write!(f, "GetValue: NonOccupiedHash: Hash {} does not exists in Table", hash),
            GetSetValueError::NonOccupiedName { name, hash } => write!(f, "GetValue: NonOccupiedName: Name {} does not exists in Scheme with hash {}", name, hash),
            GetSetValueError::SetValueWithLedger { name, hash } => write!(f, "SetValue: SetValueWithLedger: Name {} with Scheme hash {} is Ledger value", name, hash),
            GetSetValueError::TypeMismatch { name, hash, expected, received } => write!(f, "SetValue: TypeMismatch: Name {} with Scheme hash {} ExpectedType: {}, ReceivedType: {}", name, hash, expected, received),
        }
    }
}

impl fmt::Display for OperationError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::Access(e) => write!(f, "OperationError: {}", e),
            OperationError::TypeMismatch { name, hash, expected, received } => write!(f, "OperationError: TypeMismatch: Name {} with Scheme hash {} ExpectedType: {}, ReceivedType: {}", name, hash, expected, received),
            OperationError::LedgerMismatch { expected, received } => write!(f, "OperationError: LedgerMismatch: ExpectedLedger: {}, ReceivedLedger: {}", expected, received),
        }
    }
}
