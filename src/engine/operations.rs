use std::fmt;

use super::types::Value;
use super::error_types::OperationError;

pub enum ArithmeticOp {
    Sub,
    Add,
    Div,
    Mul
}

pub struct FieldRef<'a> {
    pub hash: u64,
    pub name: &'a str,
}

pub enum Operation<'a> {
    Arith {
        target: FieldRef<'a>,
        op: ArithmeticOp,
        amount: Value
    },

    Transfer {
        from: FieldRef<'a>,
        to: FieldRef<'a>,
        amount: Value
    }
}

impl fmt::Display for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticOp::Sub => write!(f, "Sub (-)"),
            ArithmeticOp::Add => write!(f, "Add (+)"),
            ArithmeticOp::Div => write!(f, "Div (/)"),
            ArithmeticOp::Mul => write!(f, "Mul (*)"),
        }
    }
}

impl Value {
    pub fn transfer_amount(&mut self, to: &mut Value, amount: &Value) -> Result<(), OperationError> {
        let amount_i128 = match amount {
            Value::UInt(a) => {
                i128::try_from(*a).map_err(|_| OperationError::AmountOverflow{amount: *a})?
            }
            other => return Err(OperationError::InvalidType{received: other.value_type()}),
        };

        match (self, to) {
            (Value::Int(f), Value::Int(t)) => {
                *f = f.checked_sub(amount_i128).ok_or_else(|| OperationError::BalanceOverflow)?;
                *t = t.checked_add(amount_i128).ok_or_else(|| OperationError::BalanceOverflow)?;
            },

            (f, t) => {
                return Err(OperationError::OperationTransferTypeMismatch { expected: f.value_type(), received: t.value_type() })
            }
        }

        Ok(())
    }

    pub fn arith_amount(&mut self, amount: &Value, op: &ArithmeticOp) -> Result<(), OperationError> {
        if self.value_type() != amount.value_type() {
            return Err(OperationError::TargetAmountTypeMismatch { expected: self.value_type(), received: amount.value_type() });
        }

        match (op) {
            ArithmeticOp::Sub => {
                match (self, amount) {
                    (Value::Int(t), Value::Int(a)) => {
                        *t = t.checked_sub(*a).ok_or_else(|| OperationError::BalanceOverflow)?;
                    },

                    (Value::UInt(t), Value::UInt(a)) => {
                        *t = t.checked_sub(*a).ok_or_else(|| OperationError::BalanceOverflow)?;
                    },

                    (Value::Float(t), Value::Float(a)) => {
                        let res = t.algebraic_sub(*a);

                        if res.is_finite() {
                            *t = res;
                        } else {
                            return Err(OperationError::BalanceOverflow);
                        }
                    },

                    (t, a) => {
                        return Err(OperationError::UnresolvedValues { value_type: t.value_type(), operation: ArithmeticOp::Sub});
                    }
                }
            },

            ArithmeticOp::Add => {
                match (self, amount) {
                    (Value::Int(t), Value::Int(a)) => {
                        *t = t.checked_add(*a).ok_or_else(|| OperationError::BalanceOverflow)?;
                    },

                    (Value::UInt(t), Value::UInt(a)) => {
                        *t = t.checked_add(*a).ok_or_else(|| OperationError::BalanceOverflow)?;
                    },

                    (Value::Float(t), Value::Float(a)) => {
                        let res = t.algebraic_add(*a);

                        if res.is_finite() {
                            *t = res;
                        } else {
                            return Err(OperationError::BalanceOverflow);
                        }
                    },

                    (t, a) => {
                        return Err(OperationError::UnresolvedValues { value_type: t.value_type(), operation: ArithmeticOp::Add});
                    }
                }
            },

            ArithmeticOp::Div => {
                match (self, amount) {
                    (Value::Float(t), Value::Float(a)) => {
                        let res = t.algebraic_sub(*a);

                        if res.is_finite() {
                            *t = res;
                        } else {
                            return Err(OperationError::BalanceOverflow);
                        }
                    },

                    (t, a) => {
                        return Err(OperationError::UnresolvedValues { value_type: t.value_type(), operation: ArithmeticOp::Div});
                    }
                }
            },

            ArithmeticOp::Mul => {
                match (self, amount) {
                    (Value::Int(t), Value::Int(a)) => {
                        *t = t.checked_mul(*a).ok_or_else(|| OperationError::BalanceOverflow)?;
                    },

                    (Value::UInt(t), Value::UInt(a)) => {
                        *t = t.checked_mul(*a).ok_or_else(|| OperationError::BalanceOverflow)?;
                    },

                    (Value::Float(t), Value::Float(a)) => {
                        let res = t.algebraic_mul(*a);

                        if res.is_finite() {
                            *t = res;
                        } else {
                            return Err(OperationError::BalanceOverflow);
                        }
                    },

                    (t, a) => {
                        return Err(OperationError::UnresolvedValues { value_type: t.value_type(), operation: ArithmeticOp::Mul});
                    }
                }
            }
        }

        Ok(())
    }
}
