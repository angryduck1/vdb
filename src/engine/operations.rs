use super::types::Value;

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
        amount: usize
    }
}

impl Value {
    pub fn transfer_amount(&mut self, to: &mut Value, amount: usize) {

    }
}
