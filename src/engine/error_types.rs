use super::types::ValueType;
use super::operations::ArithmeticOp;
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

#[derive(Debug, Eq, PartialEq)]
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

pub enum GetValueError {
    NonOccupiedHash {
        hash: u64,
    },

    NonOccupiedName {
        name: String,
        hash: u64,
    },
}

pub enum SetValueError {
    Access(GetValueError),

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
    Access(GetValueError),

    SetValueWithLedger {
        hash: u64,
    },

    InvalidType {
        received: ValueType,
    },

    TypeMismatch {
        name: String,
        hash: u64,
        expected: ValueType,
        received: ValueType,
    },

    OperationTransferTypeMismatch {
        expected: ValueType,
        received: ValueType,
    },

    LedgerMismatch {
        expected: NonZeroU32,
        received: NonZeroU32,
    },

    LedgerIsNone {
        name: String,
        hash: u64,
    },

    EqualHashs {
        hash: u64
    },

    AmountOverflow {
        amount: u128
    },

    BalanceOverflow,

    TargetAmountTypeMismatch {
        expected: ValueType,
        received: ValueType,
    },

    UnresolvedValues {
        value_type: ValueType,
        operation: ArithmeticOp
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

impl fmt::Display for GetValueError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetValueError::NonOccupiedHash { hash } => write!(f, "GetValue: NonOccupiedHash: Hash {} does not exists in Table", hash),
            GetValueError::NonOccupiedName { name, hash } => write!(f, "GetValue: NonOccupiedName: Name {} does not exists in Scheme with hash {}", name, hash),
        }
    }
}

impl fmt::Display for SetValueError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetValueError::Access(e) => write!(f, "GetValueError: {}", e),
            SetValueError::SetValueWithLedger { name, hash } => write!(f, "SetValue: SetValueWithLedger: Name {} with Scheme hash {} is Ledger value", name, hash),
            SetValueError::TypeMismatch { name, hash, expected, received } => write!(f, "SetValue: TypeMismatch: Name {} with Scheme hash {} ExpectedType: {}, ReceivedType: {}", name, hash, expected, received),
        }
    }
}

impl fmt::Display for OperationError  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::Access(e) => write!(f, "OperationError: {}", e),
            OperationError::SetValueWithLedger { hash } => write!(f, "OperationError: SetValueWithLedger: Schema with hash {} is Ledger value", hash),
            OperationError::InvalidType { received } => write!(f, "OperationError: InvalidType: Amount ExpectedType: UInt, ReceivedType: {}", received),
            OperationError::TypeMismatch { name, hash, expected, received } => write!(f, "OperationError: TypeMismatch: Name {} with Scheme hash {} ExpectedType: {}, ReceivedType: {}", name, hash, expected, received),
            OperationError::LedgerMismatch { expected, received } => write!(f, "OperationError: LedgerMismatch: ExpectedLedger: {}, ReceivedLedger: {}", expected, received),
            OperationError::LedgerIsNone { name, hash } => write!(f, "OperationError: LedgerIsNone: Name {} with Scheme hash {} has None Ledger", name, hash),
            OperationError::EqualHashs { hash } => write!(f, "OperationError: EqualHashs: From Hash {} is equal with To Hash", hash),
            OperationError::AmountOverflow { amount } => write!(f, "OperationError: AmountOverflow: Fail convert u128: {} to i128", amount),
            OperationError::BalanceOverflow => write!(f, "OperationError: BalanceOverflow"),
            OperationError::OperationTransferTypeMismatch { expected, received } => write!(f, "OperationError: OperationTransferTypeMismatch: ExpectedType: {}, ReceivedType: {}", expected, received),
            OperationError::TargetAmountTypeMismatch { expected, received } => write!(f, "OperationError: TargetAmountTypeMismatch: ExpectedType: {}, ReceivedType: {}", expected, received),
            OperationError::UnresolvedValues { value_type, operation } => write!(f, "OperationError: UnresolvedValues: Operation: {} unresolved for {}", operation, value_type),
        }
    }
}

impl From<GetValueError> for SetValueError {
    fn from(value: GetValueError) -> Self {
        SetValueError::Access(value)
    }
}

impl From<GetValueError> for OperationError {
    fn from(value: GetValueError) -> Self {
        OperationError::Access(value)
    }
}
