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
                return Err(OperationError::OperationTypeMismatch { expected: f.value_type(), received: t.value_type() })
            }
        }

        Ok(())
    }
}
