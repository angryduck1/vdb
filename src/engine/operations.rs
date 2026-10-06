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
